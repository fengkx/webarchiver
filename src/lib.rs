use std::time::Duration;

use futures::{stream, StreamExt};
use log::{debug, error, info};
use quick_xml::de::from_str;
use reqwest::{self};
use serde::Deserialize;
use tokio::fs;

#[derive(Debug, Deserialize, PartialEq)]
struct UrlSet {
    #[serde(rename = "url")]
    pub urls: Vec<UrlTag>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct UrlTag {
    pub loc: String,
}

#[derive(Debug)]
pub struct ArchiveOpts {
    pub save_outlinks: bool,
    pub save_error: bool,
    pub save_screenshot: bool,
    pub concurrency: usize,
    pub sleep_secs: u64,
}

impl Default for ArchiveOpts {
    fn default() -> Self {
        Self {
            save_outlinks: false,
            save_error: true,
            save_screenshot: false,
            concurrency: 10,
            sleep_secs: 30,
        }
    }
}

pub fn extract_urls(sitemap_str: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let sitemap: UrlSet = from_str(sitemap_str)?;
    let urls = sitemap
        .urls
        .into_iter()
        .map(|url_tag| url_tag.loc)
        .collect();
    Ok(urls)
}

pub async fn submit_urls(
    client: &reqwest::Client,
    urls: &[String],
    archive_opts: &ArchiveOpts,
) -> Result<(), Box<dyn std::error::Error>> {
    let responses = stream::iter(urls)
        .map(|url| async move {
            let mut form = reqwest::multipart::Form::new().text("url", url.clone());
            if archive_opts.save_outlinks {
                form = form.text("capture_outlinks", "on");
            }
            if archive_opts.save_error {
                form = form.text("capture_all", "on");
            }
            if archive_opts.save_screenshot {
                form = form.text("capture_screenshot", "on");
            }
            let resp = client
                .post(format!("https://web.archive.org/save/{}", url))
                .multipart(form)
                .send()
                .await;
            debug!(
                "Sleep for {} seconds to prevent rate limit",
                archive_opts.sleep_secs
            );
            tokio::time::sleep(Duration::from_secs(archive_opts.sleep_secs)).await;
            (resp, url)
        })
        .buffer_unordered(archive_opts.concurrency);
    responses
        .for_each(|(resp, url)| async move {
            match resp {
                Ok(resp) => {
                    info!(
                        "archive {}: {}",
                        urlencoding::decode(url).unwrap(),
                        resp.status()
                    );
                }
                Err(err) => {
                    error!("{}", err)
                }
            }
        })
        .await;
    Ok(())
}

pub async fn get_xml_str(
    client: &reqwest::Client,
    sitemap_uri: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    if sitemap_uri.starts_with("http") {
        let body = client.get(sitemap_uri).send().await?;
        let text = body.text().await?;
        Ok(text)
    } else {
        let text = fs::read_to_string(sitemap_uri).await?;
        Ok(text)
    }
}
