use std::collections::HashMap;
use std::fmt;

use crate::parser::ast::*;
use crate::utils::*;

mod error;

use error::EvalErrorType::*;
use error::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

trait Name {
    fn name(&self) -> String;
}

impl Name for Value {
    fn name(&self) -> String {
        match self {
            Value::Number(_) => "number".to_owned(),
            Value::String(_) => "string".to_owned(),
        }
    }
}

type Scope = HashMap<String, Value>;

type StackFrame = String;

struct Context<'a> {
    stack: Vec<StackFrame>,
    parent: Option<&'a Context<'a>>,
    scope: Scope,
}

impl Value {
    pub fn from_number(n: f64) -> Value {
        Value::Number(n)
    }

    pub fn from_string(s: &str) -> Value {
        Value::String(s.to_owned())
    }
}

type EvalResult<T> = Result<T, EvalError>;
type Eval = EvalResult<Value>;

fn eval_error<T>(error_type: EvalErrorType, pos: Pos) -> EvalResult<T> {
    Err(EvalError {
        error_type: error_type,
        pos: pos,
    })
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            stack: vec![],
            parent: None,
            scope: Scope::new(),
        }
    }

    pub fn with_parent(ctx: &'a Context) -> Self {
        Context {
            stack: ctx.stack.clone(),
            parent: Some(ctx),
            scope: Scope::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        if let Some(x) = self.scope.get(key) {
            Some(x.clone())
        } else if let Some(p) = self.parent {
            p.get(key)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.scope.insert(key.to_owned(), value);
    }
}

fn eval_literal(lit: &Literal) -> Eval {
    match lit {
        Literal::Number(n) => Ok(Value::Number(*n)),
        Literal::String(s) => Ok(Value::from_string(s)),
    }
}

fn get_number(value: Value, pos: Pos) -> EvalResult<f64> {
    match value {
        Value::Number(n) => Ok(n),
        v => return eval_error(TypeMismatch("number".to_owned(), v.name()), pos),
    }
}

fn get_string(value: Value, pos: Pos) -> EvalResult<String> {
    match value {
        Value::String(s) => Ok(s),
        v => return eval_error(TypeMismatch("string".to_owned(), v.name()), pos),
    }
}

fn eval_binary(op: BinOp, lhs_expr: &Expr, rhs_expr: &Expr, ctx: &mut Context) -> Eval {
    let lhs = eval_expression(lhs_expr, ctx)?;
    let rhs = eval_expression(rhs_expr, ctx)?;

    let lhs = get_number(lhs, lhs_expr.pos())?;
    let rhs = get_number(rhs, rhs_expr.pos())?;

    match op {
        BinOp::Mul => Ok(Value::Number(lhs * rhs)),
        BinOp::Div => Ok(Value::Number(lhs / rhs)),
        BinOp::Add => Ok(Value::Number(lhs + rhs)),
        BinOp::Sub => Ok(Value::Number(lhs - rhs)),
    }
}

fn eval_unary(op: UnOp, expr: &Expr, ctx: &mut Context) -> Eval {
    let value = eval_expression(expr, ctx)?;

    let value = get_number(value, expr.pos())?;

    match op {
        UnOp::Neg => Ok(Value::from_number(value * -1.0)),
    }
}

fn eval_expression(expr: &Expr, ctx: &mut Context) -> Eval {
    match expr {
        Expr::Name(n, r) => match ctx.get(n) {
            Some(value) => Ok(value),
            None => return eval_error(VariableNotDefined(n.clone()), r.start),
        },
        Expr::Literal(lit, _) => eval_literal(lit),
        Expr::Binary(ref lhs, op, ref rhs, _) => eval_binary(op.clone(), lhs, rhs, ctx),
        Expr::Unary(op, ref expr, _) => eval_unary(op.clone(), expr, ctx),
        Expr::Grouping(ref expr) => eval_expression(expr, ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use crate::parser;

    fn check_expression(line: &str, expected: Value) {
        let tokens = lexer::lex(&line.to_owned()).unwrap();
        let expr = parser::parse_expression(tokens).unwrap();
        let output = eval_expression(&expr, &mut Context::new()).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn context_empty() {
        let context = Context::new();
        assert_eq!(context.scope.len(), 0);
    }

    #[test]
    fn adding_item_to_context() {
        let mut context = Context::new();
        context.set("foo", Value::from_string("bar"));
        assert_eq!(context.get("foo"), Some(Value::from_string("bar")));
    }

    #[test]
    fn literals() {
        check_expression("1", Value::Number(1.0));
    }

    #[test]
    fn binary_ops() {
        check_expression("1 + 1", Value::Number(2.0));
        check_expression("1.5 + 1", Value::Number(2.5));
        check_expression("2 * 2", Value::Number(4.0));
        check_expression("4 / 2", Value::Number(2.0));
        check_expression("6 - 3", Value::Number(3.0));
        check_expression("3 * (2 + -4) / 2 * 3", Value::Number(-9.0));
    }
}
