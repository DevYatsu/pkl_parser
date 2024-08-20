# pkl-parser

`pkl-parser` is a Rust crate that provides a complete parser for Apple's Pkl language.
This crate is designed to be simple to use, supporting the full Pkl syntax with robust error handling.


## Features

- **Comprehensive Syntax Support**: Supports all syntax elements of the Pkl language.
- **Pratt Parsing**: Includes a Pratt parser for expression parsing with operator precedence.
- **Ease of Integration**: Simple API to parse and handle Pkl code in your Rust projects.

## Installation

When in your rust project, simply run: `cargo add pkl-parser`

## Usage

Here's an example of how to parse a PKL string:

```rust
use pkl_parser::{parse, pratt, Rule, Error, Pairs};

fn main() -> Result<Pairs<Rule>, Error<Rule>> {
    let source = "//some pkl code";

    let pairs = parse(source)?;
    let file = result.next().unwrap().into_inner();

    for element in file {
        match element.as_rule() {
            // then take care of each stmt separately
            Rule::stmt => (),
            // then take care of each comment/annotation separately
            Rule::COMMENT => (),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(())
}
```

### LICENSE

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
