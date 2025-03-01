#![cfg(not(target_arch = "wasm32"))]
mod support;
use futures_util::stream::StreamExt;
use support::*;

#[tokio::test]
async fn text_part() {
    let _ = env_logger::try_init();

    let form = reqwestplus::multipart::Form::new().text("foo", "bar");

    let expected_body = format!(
        "\
         --{0}\r\n\
         Content-Disposition: form-data; name=\"foo\"\r\n\r\n\
         bar\r\n\
         --{0}--\r\n\
         ",
        form.boundary()
    );

    let ct = format!("multipart/form-data; boundary={}", form.boundary());

    let server = server::http(move |mut req| {
        let ct = ct.clone();
        let expected_body = expected_body.clone();
        async move {
            assert_eq!(req.method(), "POST");
            assert_eq!(req.headers()["content-type"], ct);
            assert_eq!(
                req.headers()["content-length"],
                expected_body.len().to_string()
            );

            let mut full: Vec<u8> = Vec::new();
            while let Some(item) = req.body_mut().next().await {
                full.extend(&*item.unwrap());
            }

            assert_eq!(full, expected_body.as_bytes());

            http::Response::default()
        }
    });

    let url = format!("http://{}/multipart/1", server.addr());

    let res = reqwestplus::Client::new()
        .post(&url)
        .multipart(form)
        .send()
        .await
        .unwrap();

    assert_eq!(res.url().as_str(), &url);
    assert_eq!(res.status(), reqwestplus::StatusCode::OK);
}

#[cfg(feature = "stream")]
#[tokio::test]
async fn stream_part() {
    use futures_util::{future, stream};

    let _ = env_logger::try_init();

    let stream =
        reqwestplus::Body::wrap_stream(stream::once(future::ready(Ok::<_, reqwestplus::Error>(
            "part1 part2".to_owned(),
        ))));
    let part = reqwestplus::multipart::Part::stream(stream);

    let form = reqwestplus::multipart::Form::new()
        .text("foo", "bar")
        .part("part_stream", part);

    let expected_body = format!(
        "\
         --{0}\r\n\
         Content-Disposition: form-data; name=\"foo\"\r\n\
         \r\n\
         bar\r\n\
         --{0}\r\n\
         Content-Disposition: form-data; name=\"part_stream\"\r\n\
         \r\n\
         part1 part2\r\n\
         --{0}--\r\n\
         ",
        form.boundary()
    );

    let ct = format!("multipart/form-data; boundary={}", form.boundary());

    let server = server::http(move |mut req| {
        let ct = ct.clone();
        let expected_body = expected_body.clone();
        async move {
            assert_eq!(req.method(), "POST");
            assert_eq!(req.headers()["content-type"], ct);
            assert_eq!(req.headers()["transfer-encoding"], "chunked");

            let mut full: Vec<u8> = Vec::new();
            while let Some(item) = req.body_mut().next().await {
                full.extend(&*item.unwrap());
            }

            assert_eq!(full, expected_body.as_bytes());

            http::Response::default()
        }
    });

    let url = format!("http://{}/multipart/1", server.addr());

    let client = reqwestplus::Client::new();

    let res = client
        .post(&url)
        .multipart(form)
        .send()
        .await
        .expect("Failed to post multipart");
    assert_eq!(res.url().as_str(), &url);
    assert_eq!(res.status(), reqwestplus::StatusCode::OK);
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_file_part() {
    let _ = env_logger::try_init();

    let form = reqwestplus::blocking::multipart::Form::new()
        .file("foo", "Cargo.lock")
        .unwrap();

    let fcontents = std::fs::read_to_string("Cargo.lock").unwrap();

    let expected_body = format!(
        "\
         --{0}\r\n\
         Content-Disposition: form-data; name=\"foo\"; filename=\"Cargo.lock\"\r\n\
         Content-Type: application/octet-stream\r\n\r\n\
         {1}\r\n\
         --{0}--\r\n\
         ",
        form.boundary(),
        fcontents
    );

    let ct = format!("multipart/form-data; boundary={}", form.boundary());

    let server = server::http(move |mut req| {
        let ct = ct.clone();
        let expected_body = expected_body.clone();
        async move {
            assert_eq!(req.method(), "POST");
            assert_eq!(req.headers()["content-type"], ct);
            // files know their exact size
            assert_eq!(
                req.headers()["content-length"],
                expected_body.len().to_string()
            );

            let mut full: Vec<u8> = Vec::new();
            while let Some(item) = req.body_mut().next().await {
                full.extend(&*item.unwrap());
            }

            assert_eq!(full, expected_body.as_bytes());

            http::Response::default()
        }
    });

    let url = format!("http://{}/multipart/2", server.addr());

    let res = reqwestplus::blocking::Client::new()
        .post(&url)
        .multipart(form)
        .send()
        .unwrap();

    assert_eq!(res.url().as_str(), &url);
    assert_eq!(res.status(), reqwestplus::StatusCode::OK);
}
