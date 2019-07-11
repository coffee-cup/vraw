use std::fmt;

use crate::utils::*;

#[derive(Debug, PartialEq)]
pub struct Error<T: fmt::Display> {
    pub error_type: T,
    pub pos: Pos,
}

impl<T: fmt::Display> Error<T> {
    pub fn new(error_type: T, pos: Pos) -> Error<T> {
        Error {
            error_type: error_type,
            pos: pos,
        }
    }
}

impl<T: fmt::Display> HasPos for Error<T> {
    fn pos(&self) -> Pos {
        self.pos
    }
}

impl<T: fmt::Display> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}:{}]: {}",
            self.pos.line, self.pos.column, self.error_type
        )
    }
}
