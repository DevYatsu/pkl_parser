use std::ops::Range;

use pest;
mod ast;
mod error;
mod parser;

pub use ast::parse_as_ast;

pub use error::{PklError, PklResult};
pub use parser::parse_as_pairs;
pub use parser::pratt;
pub use parser::PklParser;
pub use parser::Rule;
pub use pest::error::Error as PestError;
pub use pest::iterators::{Pair, Pairs};

pub type Span = Range<usize>;
