use std::fmt;

use crate::error::Error;
use crate::utils::*;

#[derive(Debug, PartialEq, Clone)]
pub enum EvalErrorType {
    VariableNotDefined(String),
    ShapeNotDefined(String),
    TypeMismatch(String, String),
    SvgExpectsString(String),
    ShapeAlreadyDefined(String),
    NumArgs(String, usize, usize),
    InvalidArgName(String, String),
    MissingArgs(String, Vec<String>),
    UnExpectedArg(String, String),
    MissingRequiredArg(String, String),
    StackOverflow(Vec<String>),
    MissingMain,
    StdLibNotLoaded(String),
}

impl fmt::Display for EvalErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalErrorType::VariableNotDefined(name) => write!(f, "Variable `{}` not defined", name),
            EvalErrorType::ShapeNotDefined(name) => write!(f, "Shape `{}` not defined", name),
            EvalErrorType::TypeMismatch(expected, received) => write!(
                f,
                "Typemismatch. Expected: {}, Received: {}",
                expected, received
            ),
            EvalErrorType::SvgExpectsString(received) => write!(
                f,
                "svg value arg needs to be a string. Received {}",
                received
            ),
            EvalErrorType::ShapeAlreadyDefined(name) => write!(f, "Shape {} already defined", name),
            EvalErrorType::NumArgs(func, expected, received) => write!(
                f,
                "Incorrect number of args to {}. Expected: {}, Received: {}",
                func, expected, received
            ),
            EvalErrorType::InvalidArgName(func, arg) => {
                write!(f, "{} does not have an arg named {}", func, arg)
            }
            EvalErrorType::MissingArgs(func, args) => {
                write!(f, "Missing args {} for {}", args.join(", "), func)
            }
            EvalErrorType::UnExpectedArg(func, arg) => {
                write!(f, "Unexpected arg {} to {}", arg, func)
            }
            EvalErrorType::MissingRequiredArg(func, arg) => {
                write!(f, "Missing required arg {} to {}", arg, func)
            }
            EvalErrorType::MissingMain => write!(f, "Missing main shape"),
            EvalErrorType::StackOverflow(stack) => {
                write!(f, "Stack overflow\n{}", stack.join("\n    "))
            }
            EvalErrorType::StdLibNotLoaded(err) => write!(f, "Error {} stdlib", err),
        }
    }
}

pub type EvalError = Error<EvalErrorType>;
