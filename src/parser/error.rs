use std::fmt;

use crate::utils::*;

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    UnExpectedEndOfInput,
    IdentiferCannotBeReservedWord(String),
    UnBalancedParen,
    Expected(String, Option<String>),
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    error_type: ParseErrorType,
    pos: Pos,
}

impl HasPos for ParseError {
    fn pos(&self) -> Pos {
        self.pos
    }
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErrorType::UnExpectedEndOfInput => write!(f, "Unexpected end of input."),
            ParseErrorType::IdentiferCannotBeReservedWord(id) => {
                write!(f, "Identifier {} cannot be a reserved word.", id)
            }
            ParseErrorType::UnBalancedParen => write!(f, "Unbalanced paren."),
            ParseErrorType::Expected(expected, oFound) => match oFound {
                Some(found) => write!(f, "Expected {}. Found {}.", expected, found),
                None => write!(f, "Expected {}", expected),
            },
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}:{}] {}",
            self.pos.line, self.pos.column, self.error_type
        )
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse_error<T>(error_type: ParseErrorType, pos: Pos) -> ParseResult<T> {
    Err(ParseError {
        error_type: error_type,
        pos: pos,
    })
}
