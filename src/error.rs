use std::fmt;

use crate::utils::*;

#[derive(Debug, PartialEq)]
pub struct Error {
    message: String,
    pos: Option<Pos>,
}

impl Error {
    pub fn new(message: String, pos: Option<Pos>) -> Error {
        Error {
            message: message,
            pos: pos,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.pos {
            Some(p) => write!(f, "[{}:{}]: {}", p.line, p.column, self.message),
            _ => write!(f, "{}", self.message),
        }
    }
}
