use crate::utils::*;

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    UnExpectedEndOfInput,
    IdentiferCannotBeReservedWord(String),
}

pub struct ParseError {
    error_type: ParseErrorType,
    pos: Pos,
}
