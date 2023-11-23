//! Settings for impersonating the Chrome browser

use crate::ClientBuilder;

mod ver;

pub(crate) fn configure_chrome(ver: ChromeVersion, builder: ClientBuilder) -> ClientBuilder {
    let settings = ver::get_config_from_ver(ver);

    builder
        .use_boring_tls(settings.tls_builder_func)
        .http2_initial_stream_window_size(settings.http2.initial_stream_window_size)
        .http2_initial_connection_window_size(settings.http2.initial_connection_window_size)
        .http2_max_concurrent_streams(settings.http2.max_concurrent_streams)
        .http2_max_header_list_size(settings.http2.max_header_list_size)
        .http2_header_table_size(settings.http2.header_table_size)
        .http2_enable_push(settings.http2.enable_push)
        .replace_default_headers(settings.headers)
        .brotli(settings.brotli)
        .gzip(settings.gzip)
}

/// Defines the Chrome version to mimic when setting up a builder
#[derive(Debug)]
#[allow(missing_docs)]
pub enum ChromeVersion {
    V104,
    V105,
    V106,
    V108,
    V110,
    V117,
    V118,
    V119,
    Latest,
}

impl ChromeVersion {
    pub fn get_useragent(&self) -> String {
        get_chrome_useragent(self)
    }
}

pub fn get_chrome_useragent(version: &ChromeVersion) -> String {
    match version {
        ChromeVersion::V104 => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36",
        ),
        ChromeVersion::V105 => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36",
        ),
        ChromeVersion::V106 => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36",
        ),
        ChromeVersion::V108 => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36",
        ),
        ChromeVersion::V110 => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36",
        ),
        ChromeVersion::V117 => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36",
        ),
        ChromeVersion::V118 => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36",
        ),
        ChromeVersion::V119 => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        ),
        ChromeVersion::Latest => String::from(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        ),
    }
}
