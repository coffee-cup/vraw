use std::fmt;

use crate::error::Error;
use crate::utils::*;

#[derive(Debug, PartialEq)]
pub enum LexerErrorType {
    InvalidIdentifier,
    StringNeverTerminated,
    UnexpectedCharacter(char),
}

impl fmt::Display for LexerErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerErrorType::InvalidIdentifier => write!(f, "Identifier must start with a letter"),
            LexerErrorType::StringNeverTerminated => write!(f, "String is never terminated"),
            LexerErrorType::UnexpectedCharacter(c) => write!(f, "Unexpected character `{}`", c),
        }
    }
}

pub type LexerError = Error<LexerErrorType>;
