use std::fmt;

use crate::utils::*;

#[derive(Debug, PartialEq)]
pub enum EvalErrorType {
    VariableNotDefined(String),
    TypeMismatch(String, String),
}

#[derive(Debug, PartialEq)]
pub struct EvalError {
    pub error_type: EvalErrorType,
    pub pos: Pos,
}

impl HasPos for EvalError {
    fn pos(&self) -> Pos {
        self.pos
    }
}

impl fmt::Display for EvalErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalErrorType::VariableNotDefined(name) => write!(f, "Variable `{}` not defined", name),
            EvalErrorType::TypeMismatch(expected, received) => write!(
                f,
                "Typemismatch. Expected: {}, Received: {}",
                expected, received
            ),
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}:{}] {}",
            self.pos.line, self.pos.column, self.error_type
        )
    }
}
