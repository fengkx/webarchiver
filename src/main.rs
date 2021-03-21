use clap::{App, AppSettings, Arg};
use log::{info, LevelFilter};
use reqwest::header::{self, USER_AGENT};
use reqwest::{self};
use webarchiver::{extract_urls, get_xml_str, submit_urls, ArchiveOpts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
    let matches = build_cli().get_matches();
    let concurrency = matches
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    info!("concurrency request ammount: {}", concurrency);
    let sleep_secs = matches
        .value_of("sleep_secs")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    info!(
        "Sleep for {} seconds to prevent rate limit every batch",
        sleep_secs
    );
    let sitemap_uri = matches.value_of("sitemap_uri").unwrap();
    info!("Sitemap: {}", sitemap_uri);

    let mut headers = header::HeaderMap::new();
    headers.insert(USER_AGENT, header::HeaderValue::from_static("webarchiver"));
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let client = &client;

    let xml_str = get_xml_str(client, &sitemap_uri).await?;
    let urls = extract_urls(&xml_str)?;
    info!("Total url count: {}", urls.len());
    let archive_opts = ArchiveOpts {
        concurrency,
        sleep_secs,
        ..Default::default()
    };
    submit_urls(client, &urls, &archive_opts).await.unwrap();
    Ok(())
}

fn build_cli() -> App<'static, 'static> {
    App::new("Web Archiver")
        .version("0.1")
        .author("fengkx https://github.com/fengkx/webarchiver")
        .about("Save all url in a sitemap to archive.org Wayback Machine")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("concurrency")
                .short("c")
                .long("concurrency")
                .takes_value(true)
                .help("concurrency request number")
                .default_value("4")
                .validator(|v| match v.parse::<usize>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Expected a non-negative number but got `{}`", v)),
                }),
        )
        .arg(
            Arg::with_name("sleep_secs")
                .short("s")
                .long("sleep")
                .takes_value(true)
                .help("sleep timeout in seconds for prevent rate limit")
                .default_value("30")
                .validator(|v| match v.parse::<u64>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Expected a non-negative number but got `{}`", v)),
                }),
        )
        .arg(
            Arg::with_name("sitemap_uri")
                .takes_value(true)
                .value_name("FILE / URL")
                .help("Sitemap path or url")
                .required(true),
        )
}
