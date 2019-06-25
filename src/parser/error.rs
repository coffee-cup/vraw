use crate::utils::*;

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    UnExpectedEndOfInput,
    IdentiferCannotBeReservedWord(String),
    UnBalancedParen,
    Expected(String, Option<String>),
    NoOperator,
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

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse_error<T>(error_type: ParseErrorType, pos: Pos) -> ParseResult<T> {
    Err(ParseError {
        error_type: error_type,
        pos: pos,
    })
}
