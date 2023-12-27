use crate::ClientBuilder;

mod ver;

pub(crate) fn configure_safari(ver: SafariVersion, builder: ClientBuilder) -> ClientBuilder {
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
pub enum SafariVersion {
    V17_2,
    Latest
}

impl SafariVersion {
    pub fn get_useragent(&self) -> String {
        get_safari_useragent(self)
    }
}

pub fn get_safari_useragent(version: &SafariVersion) -> String {
    match version {
        SafariVersion::V17_2 => String::from(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
        ),
        SafariVersion::Latest => String::from(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
        ),
    }
}