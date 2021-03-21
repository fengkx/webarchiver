# webarchiver

> Save all page in a sitemap.xml to [archive.org Wayback manchine](https://web.archive.org/)

## Usage

```sh
Web Archiver 0.1
fengkx https://github.com/fengkx/webarchiver
Save all url in a sitemap to archive.org Wayback Machine

USAGE:
    webarchiver [FLAGS] [OPTIONS] <FILE / URL>

FLAGS:
        --external      Save external link
    -h, --help          Prints help information
        --screenshot    Save screenshot
    -V, --version       Prints version information

OPTIONS:
    -c, --concurrency <concurrency>    concurrency request number [default: 4]
    -s, --sleep <sleep_secs>           sleep timeout in seconds for prevent rate limit [default: 30]

ARGS:
    <FILE / URL>    Sitemap path or url
```

Example: `webarchiver https://www.fengkx.top/sitemap.xml`
