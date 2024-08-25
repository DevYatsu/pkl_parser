use crate::{Rule, Span};
use std::fmt::Display;
use thiserror::Error;

/// A result type for PKL parsing operations.
///
/// The `PklResult` type is a specialized `Result` type used throughout the PKL parsing code.
/// It represents either a successful result (`T`) or a `PklError`.
pub type PklResult<T> = std::result::Result<T, PklError>;

/// Represents an error in Pkl.
#[derive(Error, Debug)]
pub struct PklError(#[from] ErrorRepr);

impl PklError {
    pub fn msg(msg: impl Into<String>) -> PklError {
        ErrorRepr::WithoutContext(msg.into()).into()
    }

    pub fn with_file_name(mut self, file_name: impl Into<String>) -> Self {
        match self.0 {
            ErrorRepr::WithContext(_, _, ref mut name) => {
                *name = Some(file_name.into());
                self
            }
            _ => self,
        }
    }
}

// Private and free to change across minor version of the crate.
#[derive(Error, Debug)]
enum ErrorRepr {
    WithContext(String, Span, Option<String>),
    WithoutContext(String),
    Parse(#[from] crate::PestError<Rule>),

    #[error(transparent)]
    Io(std::io::Error),
}

impl Display for PklError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Display for ErrorRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorRepr::Parse(e) => e.fmt(f),
            ErrorRepr::Io(io) => io.fmt(f),
            ErrorRepr::WithContext(msg, range, file_name) => {
                write!(f, "{msg} at {}..{}", range.start, range.end)?;
                if file_name.is_some() {
                    write!(f, "in file `{}`", file_name.as_ref().unwrap())?;
                }
                Ok(())
            }
            ErrorRepr::WithoutContext(msg) => {
                write!(f, "{msg} ")
            }
        }
    }
}

impl From<(String, Span)> for PklError {
    fn from(value: (String, Span)) -> Self {
        ErrorRepr::WithContext(value.0, value.1, None).into()
    }
}
impl From<(String, Span, String)> for PklError {
    fn from(value: (String, Span, String)) -> Self {
        ErrorRepr::WithContext(value.0, value.1, Some(value.2)).into()
    }
}
impl From<crate::PestError<Rule>> for PklError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        ErrorRepr::Parse(value).into()
    }
}
