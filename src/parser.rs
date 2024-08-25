use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use pest_derive::Parser;

use crate::PklResult;

/// `PklParser` is a parser generated from the `pkl.pest` grammar file.
/// This struct is used to parse input strings according to the rules defined in the grammar.
#[derive(Parser)]
#[grammar = "pkl.pest"]
pub struct PklParser;

/// Parses the given source string using the `PklParser` and returns a `Pairs` iterator over the parsed tokens.
///
/// # Arguments
///
/// * `src` - A string slice that holds the source code to be parsed.
///
/// # Returns
///
/// * `Ok(Pairs<Rule>)` - A successful result containing a `Pairs` iterator over the parsed tokens.
/// * `Err(PklError)` - A Pkl Error, with variant Parse as error occurred during parsing, encapsulated in a `pest` error.
///
/// # Example
///
/// ```ignore
/// let source = "your source code here";
/// let parsed = parse_as_pairs(source);
/// match parsed {
///     Ok(pairs) => {
///         for pair in pairs {
///             // handle parsed tokens
///         }
///     }
///     Err(error) => {
///         eprintln!("Parsing error: {:?}", error);
///     }
/// }
/// ```
pub fn parse_as_pairs(src: &str) -> PklResult<Pairs<Rule>> {
    let result = PklParser::parse(Rule::file, src)?;

    Ok(result)
}

/// Constructs a `PrattParser` for parsing and evaluating expressions with operator precedence and associativity.
///
/// The `PrattParser` is configured with various infix, prefix, and postfix operators, as specified by the PKL grammar.
/// This is used to handle expressions like arithmetic operations, comparisons, and logical operators.
///
/// # Returns
///
/// * `PrattParser<Rule>` - A configured `PrattParser` for parsing expressions with the appropriate precedence and associativity.
///
/// # Example
///
/// ```ignore
/// let pratt_parser = pratt();
/// // Use the pratt_parser to evaluate expressions
/// ```
/// // For how to use the pratt_parser, [see](https://pest.rs/book/precedence.html)
pub fn pratt() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::null_coalescing, Assoc::Left))
        .op(Op::infix(Rule::comp_equal, Assoc::Left)
            | Op::infix(Rule::and, Assoc::Left)
            | Op::infix(Rule::or, Assoc::Left)
            | Op::infix(Rule::comp_not_equal, Assoc::Left)
            | Op::infix(Rule::comp_greater, Assoc::Left)
            | Op::infix(Rule::comp_greater_equal, Assoc::Left)
            | Op::infix(Rule::comp_less, Assoc::Left)
            | Op::infix(Rule::comp_less_equal, Assoc::Left))
        .op(Op::infix(Rule::is_op, Assoc::Left) | Op::infix(Rule::as_op, Assoc::Left))
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left)
            | Op::infix(Rule::modulo, Assoc::Left)
            | Op::infix(Rule::div, Assoc::Left)
            | Op::infix(Rule::div_r, Assoc::Left))
        .op(Op::infix(Rule::pow, Assoc::Right))
        .op(Op::postfix(Rule::non_null))
        .op(Op::prefix(Rule::neg) | Op::prefix(Rule::logical_not))
}
