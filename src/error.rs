use std::fmt;

use crate::utils::*;

#[derive(Debug, PartialEq)]
pub struct Error {
    message: String,
    pos: Pos,
}

impl Error {
    pub fn new(message: String, pos: Pos) -> Error {
        Error {
            message: message,
            pos: pos,
        }
    }
}

impl HasPos for Error {
    fn pos(&self) -> Pos {
        self.pos
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}:{}]: {}",
            self.pos.line, self.pos.column, self.message
        )
    }
}
