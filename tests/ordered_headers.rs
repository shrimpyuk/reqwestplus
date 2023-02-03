#![cfg(not(target_arch = "wasm32"))]
mod support;
use http::HeaderMap;
use support::*;

use reqwestplus::Client;

#[tokio::test]
async fn insert_order() {
    let server = server::http(move |req| async move {
        let mut headers = req.headers().iter();
        assert_eq!(headers.next().unwrap().0, "hello");
        assert_eq!(headers.next().unwrap().0, "second");
        assert_eq!(headers.next().unwrap().0, "accept");
        assert_eq!(headers.next().unwrap().0, "host");
        assert_eq!(headers.next(), None);

        http::Response::default()
    });

    let mut headers = HeaderMap::new();

    headers.insert("foo", "bar".parse().unwrap());
    headers.append("foo", "bar2".parse().unwrap());
    headers.insert("hello", "world".parse().unwrap());
    headers.insert("second", "is it?".parse().unwrap());
    headers.remove("foo");
    headers.insert("accept", "*/*".parse().unwrap());

    let url = format!("http://{}/1", server.addr());
    let res = Client::builder()
        .no_proxy()
        .accept_header(false)
        .default_headers(headers)
        .build()
        .unwrap()
        .get(&url)
        .send()
        .await
        .unwrap();

    assert_eq!(res.url().as_str(), &url);
    assert_eq!(res.status(), reqwestplus::StatusCode::OK);
    assert_eq!(res.remote_addr(), Some(server.addr()));
}

#[tokio::test]
async fn header_order() {
    let server = server::http(move |req| async move {
        let mut headers = req.headers().iter();
        assert_eq!(headers.next().unwrap().0, "host");
        assert_eq!(headers.next().unwrap().0, "second");
        assert_eq!(headers.next().unwrap().0, "hello");
        assert_eq!(headers.next().unwrap().0, "accept");
        assert_eq!(headers.next(), None);

        http::Response::default()
    });

    let mut headers = HeaderMap::new();

    // You need to pass host in order for it to be in right order
    headers.insert("host", "example".parse().unwrap());

    headers.insert("foo", "bar".parse().unwrap());
    headers.append("foo", "bar2".parse().unwrap());
    headers.insert("hello", "world".parse().unwrap());
    headers.insert("second", "is it?".parse().unwrap());
    headers.remove("foo");

    let header_order = vec![
        "host".parse().unwrap(),
        "second".parse().unwrap(),
        "hello".parse().unwrap(),
        "accept".parse().unwrap(),
    ];

    let url = format!("http://{}/1", server.addr());
    let res = Client::builder()
        .no_proxy()
        .header_order(header_order)
        .default_headers(headers)
        .build()
        .unwrap()
        .get(&url)
        .send()
        .await
        .unwrap();

    assert_eq!(res.url().as_str(), &url);
    assert_eq!(res.status(), reqwestplus::StatusCode::OK);
    assert_eq!(res.remote_addr(), Some(server.addr()));
}
