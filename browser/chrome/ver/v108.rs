use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, UPGRADE_INSECURE_REQUESTS, USER_AGENT},
    HeaderMap,
};

pub fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        "sec-ch-ua",
        r#""Not?A_Brand";v="8", "Chromium";v="108", "Google Chrome";v="108""#
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());

    headers
}
