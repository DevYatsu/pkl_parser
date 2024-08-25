// in here we extract all the code present in the language reference
// and we pass it into our parser to check if it parses the code successfully

use pkl_parser::parse_as_pairs as parse;
use reqwest::blocking::Client;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION,
    DNT, PRAGMA, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};
use std::fs;

#[test]
pub fn stdlib() {
    let file_content = fs::read_to_string("stdlib_uris.txt").expect("check if file still exists");
    let file_uris = file_content
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    for uri in file_uris {
        let mut headers = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36",
            ),
        );
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(REFERER, HeaderValue::from_static("https://www.google.com/"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(DNT, HeaderValue::from_static("1")); // Do Not Track request header
        headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));

        let client = Client::new();
        let resp = client.get(uri).send().unwrap();
        assert!(resp.status().is_success());

        let src = resp.text().unwrap();

        let parsed = parse(&src);
        assert!(parsed.is_ok());
        println!("{} parsed successfully", uri);
    }
}
