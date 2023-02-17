use reqwestplus::browser::ChromeVersion;

fn main() {
    // Build a client to mimic latest chrome
    let client = reqwestplus::blocking::Client::builder()
        .chrome_builder(ChromeVersion::V110)
        .build()
        .unwrap();

    // Use the API you're already familiar with
    match client.get("https://tls.peet.ws/api/all").send() {
        Ok(res) => {
            println!("{}", res.text().unwrap());
        }
        Err(err) => {
            dbg!(err);
        }
    };
}
