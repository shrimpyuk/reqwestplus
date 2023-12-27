use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, UPGRADE_INSECURE_REQUESTS, USER_AGENT},
    HeaderMap,
};

use crate::browser::{BrowserSettings, Http2Data};
use boring::ssl::{
    CertCompressionAlgorithm, SslConnector, SslConnectorBuilder, SslMethod, SslVersion,
};
use std::sync::Arc;

pub(super) fn get_settings(headers: HeaderMap) -> BrowserSettings {
    BrowserSettings {
        tls_builder_func: Arc::new(create_ssl_connector),
        http2: Http2Data {
            initial_stream_window_size: 6291456,
            initial_connection_window_size: 4194304,
            max_concurrent_streams: 100,
            max_header_list_size: 262144,
            header_table_size: 65536,
            enable_push: Some(false),
        },
        headers,
        gzip: true,
        brotli: true,
    }
}

pub(super) fn create_ssl_connector() -> SslConnectorBuilder {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();

    builder.set_grease_enabled(true);

    builder.enable_ocsp_stapling();

    let cipher_list = [
        "TLS_AES_128_GCM_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA",
        "TLS_RSA_WITH_3DES_EDE_CBC_SHA",
    ];

    builder.set_cipher_list(&cipher_list.join(":")).unwrap();

    let sigalgs_list = [
        "ecdsa_secp256r1_sha256",
        "rsa_pss_rsae_sha256",
        "rsa_pkcs1_sha256",
        "ecdsa_secp384r1_sha384",
        "ecdsa_sha1",
        "rsa_pss_rsae_sha384",
        "rsa_pkcs1_sha384",
        "rsa_pss_rsae_sha512",
        "rsa_pkcs1_sha512",
        "rsa_pkcs1_sha1",
    ];

    builder.set_sigalgs_list(&sigalgs_list.join(":")).unwrap();

    builder.enable_signed_cert_timestamps();

    builder.set_alpn_protos(b"\x02h2\x08http/1.1").unwrap();

    builder
        .add_cert_compression_alg(CertCompressionAlgorithm::Zlib)
        .unwrap();

    builder
        .set_min_proto_version(Some(SslVersion::TLS1))
        .unwrap();

    builder
        .set_max_proto_version(Some(SslVersion::TLS1_3))
        .unwrap();

    builder
}


pub fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());

    headers
}
