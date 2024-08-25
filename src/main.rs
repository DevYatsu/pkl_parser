use pkl_parser::{parse_as_ast, parse_as_pairs, Rule};
use std::{fs, time::Instant};

fn main() {
    let src = fs::read_to_string("x.pkl").unwrap();

    let src = src.repeat(1);
    let time = Instant::now();

    let result = parse_as_ast(&src).unwrap();

    for s in result {
        println!("{s:?}");
    }

    println!(
        "{}ms to parse {} chars",
        time.elapsed().as_millis(),
        src.len()
    );
}
