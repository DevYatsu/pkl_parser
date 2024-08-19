// in here we extract all the code present in the language reference
// and we pass it into our parser to check if it parses the code successfully

use pkl_parser::parse;
use regex::Regex;
use select::{document::Document, predicate::Name};

#[test]
pub fn lang_ref_code() {
    let resp =
        reqwest::blocking::get("https://pkl-lang.org/main/current/language-reference/index.html")
            .unwrap();
    assert!(resp.status().is_success());

    let re = Regex::new(r"\(\d\)").unwrap();
    let code_parts = Document::from_read(resp)
        .unwrap()
        .find(Name("code"))
        .filter(|n| {
            // removes exprs present in the doc
            n.attr("class")
                .map(|x| !x.contains("expression"))
                .unwrap_or(false)
                && !n.text().starts_with("#")
                && !n.text().starts_with("\"")
                && n.attr("class")
                    .map(|x| x.contains("language-pkl"))
                    .unwrap_or(false)
        })
        .map(|x| re.replace_all(&x.text(), "").to_string())
        .collect::<Vec<_>>();

    assert!(!code_parts.is_empty());

    let code = code_parts.join("\n");

    let parsed = parse(&code);
    println!("{:?}", parsed);
    assert!(parsed.is_ok());
}
