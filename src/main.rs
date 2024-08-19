use pkl_parser::{parse, Rule};
use std::{fs, time::Instant};

fn main() {
    let src = fs::read_to_string("a.pkl").unwrap();

    let src = src.repeat(1);
    let time = Instant::now();

    let mut result = parse(&src).unwrap();

    let file = result.next().unwrap().into_inner();

    let mut doc_comment: Option<String> = None;
    for record in file {
        match record.as_rule() {
            Rule::stmt => {
                for stmt in record.into_inner() {
                    match stmt.as_rule() {
                        _ => {
                            // when managing the statement
                            // take the value inside the doc
                            // comment to attach it to attach
                            // it to the stmt if necessary
                            for stmt in stmt.into_inner() {
                                match stmt.as_rule() {
                                    Rule::COMMENT => {
                                        // println!("comment: {:?}", stmt.as_rule())
                                    }
                                    _ => {
                                        // println!("any stmt: {:?}", stmt.as_rule());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Rule::EOI => (),
            Rule::COMMENT => {
                for com in record.into_inner() {
                    match com.as_rule() {
                        Rule::doc_comment => {
                            doc_comment = Some(
                                doc_comment
                                    .map(|mut x| {
                                        x.push_str(com.as_str());
                                        x
                                    })
                                    .unwrap_or(String::from(com.as_str())),
                            );
                        }
                        Rule::annotation => {
                            // take care of them here
                        }
                        Rule::line_comment | Rule::multiline_comment => (),
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    println!(
        "{}ms to parse {} chars",
        time.elapsed().as_millis(),
        src.len()
    );
}
