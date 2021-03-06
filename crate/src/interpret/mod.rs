use std::collections::HashMap;
use std::fmt;

use crate::lexer;
use crate::parser;
use crate::parser::ast::*;
use crate::utils::*;

mod error;
mod stdlib;

use error::EvalErrorType::*;
use error::*;

static STACK_LIMIT: usize = 256;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
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

type StackFrame = String;

struct Context<'a> {
    stack: Vec<StackFrame>,
    parent: Option<&'a Context<'a>>,
    scope: HashMap<String, Value>,
    shapes: HashMap<String, Shape>,
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

fn eval_error<T>(error_type: EvalErrorType, pos: Pos) -> EvalResult<T> {
    Err(EvalError::new(error_type, pos))
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            stack: vec![],
            parent: None,
            scope: HashMap::new(),
            shapes: HashMap::new(),
        }
    }

    pub fn with_parent(ctx: &'a Context) -> Self {
        Context {
            stack: ctx.stack.clone(),
            parent: Some(ctx),
            scope: HashMap::new(),
            shapes: HashMap::new(),
        }
    }

    pub fn set_scope(&mut self, scope: HashMap<String, Value>) {
        self.scope = scope;
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

fn eval_literal(lit: &Literal) -> EvalResult<Value> {
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

fn is_string(value: &Value) -> bool {
    match value {
        Value::String(_) => true,
        _ => false,
    }
}

fn is_number(value: &Value) -> bool {
    match value {
        Value::Number(_) => true,
        _ => false,
    }
}

fn eval_binary(
    op: BinOp,
    lhs_expr: &Expr,
    rhs_expr: &Expr,
    ctx: &mut Context,
) -> EvalResult<Value> {
    let lhs = eval_expression(lhs_expr, ctx)?;
    let rhs = eval_expression(rhs_expr, ctx)?;

    match op {
        BinOp::Mul => {
            let lhs = get_number(lhs, lhs_expr.pos())?;
            let rhs = get_number(rhs, rhs_expr.pos())?;
            Ok(Value::Number(lhs * rhs))
        }
        BinOp::Div => {
            let lhs = get_number(lhs, lhs_expr.pos())?;
            let rhs = get_number(rhs, rhs_expr.pos())?;
            Ok(Value::Number(lhs / rhs))
        }
        BinOp::Add => {
            if !is_number(&lhs) || !is_number(&rhs) {
                Ok(Value::String(format!("{}{}", lhs, rhs)))
            } else {
                let lhs = get_number(lhs, lhs_expr.pos())?;
                let rhs = get_number(rhs, rhs_expr.pos())?;

                Ok(Value::Number(lhs + rhs))
            }
        }
        BinOp::Sub => {
            let lhs = get_number(lhs, lhs_expr.pos())?;
            let rhs = get_number(rhs, rhs_expr.pos())?;
            Ok(Value::Number(lhs - rhs))
        }
    }
}

fn eval_unary(op: UnOp, expr: &Expr, ctx: &mut Context) -> EvalResult<Value> {
    let value = eval_expression(expr, ctx)?;
    let value = get_number(value, expr.pos())?;

    match op {
        UnOp::Neg => Ok(Value::from_number(value * -1.0)),
    }
}

fn eval_expression(expr: &Expr, ctx: &mut Context) -> EvalResult<Value> {
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

fn eval_svg_call(call: &FunCall, ctx: &mut Context) -> EvalResult<Value> {
    if call.args.len() != 1 {
        return eval_error(NumArgs(call.ident.clone(), 1, call.args.len()), call.pos());
    }

    let arg = &call.args[0];
    if arg.name != "value" {
        return eval_error(
            MissingArgs(call.ident.clone(), vec!["value".to_owned()]),
            call.pos(),
        );
    }

    let value = eval_expression(&arg.expr, ctx)?;

    let value = match get_string(value, arg.expr.pos()) {
        Err(e) => match &e.error_type {
            TypeMismatch(_, received) => {
                return eval_error(SvgExpectsString(received.clone()), e.pos())
            }
            error_type => return eval_error(error_type.clone(), e.pos()),
        },
        Ok(v) => v,
    };

    Ok(Value::String(value))
}

fn eval_call(call: &FunCall, ctx: &mut Context) -> EvalResult<Value> {
    ctx.stack.push(call.ident.clone());
    if ctx.stack.len() > STACK_LIMIT {
        return eval_error(StackOverflow(ctx.stack.clone()), call.pos());
    }

    if call.ident == "svg" {
        return eval_svg_call(call, ctx);
    }

    let shape = match ctx.shapes.get(&call.ident) {
        Some(shape) => shape.clone(),
        None => return eval_error(ShapeNotDefined(call.ident.clone()), call.pos()),
    };

    let mut args: HashMap<String, Value> = HashMap::new();

    for shape_arg in shape.args.iter() {
        let value = match call.args.iter().find(|arg| arg.name == shape_arg.name) {
            None => {
                // use default value if it exists
                match &shape_arg.default {
                    None => {
                        return eval_error(
                            MissingRequiredArg(call.ident.clone(), shape_arg.name.clone()),
                            call.pos(),
                        )
                    }
                    Some(default_expr) => eval_expression(default_expr, ctx)?,
                }
            }
            Some(call_arg) => eval_expression(&call_arg.expr, ctx)?,
        };

        args.insert(shape_arg.name.clone(), value);
    }

    let current_scope = ctx.scope.clone();

    ctx.set_scope(args);
    let result = eval_block(&shape.block, ctx)?;
    ctx.set_scope(current_scope);

    Ok(result)
}

fn eval_block(block: &Block, ctx: &mut Context) -> EvalResult<Value> {
    let mut out: String = "".to_owned();

    for call in block.calls.iter() {
        match eval_call(call, ctx)? {
            Value::String(s) => out.push_str(s.as_str()),
            _ => panic!("call should return string value."),
        }
    }

    Ok(Value::String(out))
}

fn find_shapes(
    shapes: HashMap<String, Shape>,
    program: &Program,
    error_missing_main: bool,
) -> EvalResult<HashMap<String, Shape>> {
    let mut shapes: HashMap<String, Shape> = shapes.clone();
    let mut found_main = false;

    for decl in program.decls.iter() {
        match &decl {
            Decl::ShapeDecl(shape) => {
                if shapes.contains_key(&shape.name) {
                    return eval_error(ShapeAlreadyDefined(shape.name.clone()), shape.pos());
                }

                if shape.name == "main" {
                    found_main = true;
                }

                shapes.insert(shape.name.clone(), shape.clone());
            }
        };
    }

    if !found_main && error_missing_main {
        return eval_error(MissingMain, program.end);
    }

    Ok(shapes)
}

fn load_stdlib_shapes() -> EvalResult<HashMap<String, Shape>> {
    let stdlib_input = stdlib::get_stdlib();

    let tokens = match lexer::lex(&stdlib_input) {
        Err(_) => return eval_error(StdLibNotLoaded("lexing".to_owned()), create_pos(0, 0)),
        Ok(tokens) => tokens,
    };

    let ast = match parser::parse_program(tokens) {
        Err(_) => return eval_error(StdLibNotLoaded("parsing".to_owned()), create_pos(0, 0)),
        Ok(ast) => ast,
    };

    match find_shapes(HashMap::new(), &ast, false) {
        Err(_) => {
            return eval_error(
                StdLibNotLoaded("finding shapes for".to_owned()),
                create_pos(0, 0),
            )
        }
        Ok(shapes) => Ok(shapes),
    }
}

pub fn eval_program(program: &Program) -> EvalResult<String> {
    let ctx = &mut Context::new();

    let stdlib_shapes = load_stdlib_shapes()?;

    let shapes = find_shapes(stdlib_shapes, program, true)?;
    ctx.shapes = shapes;

    let main = ctx.shapes.get("main").unwrap().clone();

    let main_svg = match eval_block(&main.block, ctx)? {
        Value::String(value) => value,
        _ => panic!("eval_block should return Value::String"),
    };

    let wrapped_svg = format!(
        "<svg width=\"100%\" height=\"100%\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
        main_svg
    );

    Ok(wrapped_svg)
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot_matches;

    use super::*;

    fn check_expression(line: &str, expected: Value) {
        let tokens = lexer::lex(&line.to_owned()).unwrap();
        let expr = parser::parse_expression(tokens).unwrap();
        let output = eval_expression(&expr, &mut Context::new()).unwrap();

        assert_eq!(output, expected);
    }

    fn run_program(line: &str) -> EvalResult<String> {
        let tokens = lexer::lex(&line.to_owned()).unwrap();
        let program = parser::parse_program(tokens).unwrap();

        let ctx = &mut Context::new();
        eval_program(&program)
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
        check_expression("3 + \"hello\"", Value::String("3hello".to_owned()))
    }

    #[test]
    fn simple_find_shapes() {
        let line = "
shape main() {}
shape circle(r) {}
shape rect(w, h) {}
";
        let tokens = lexer::lex(&line.to_owned()).unwrap();
        let program = parser::parse_program(tokens).unwrap();

        let shapes = find_shapes(HashMap::new(), &program, true).unwrap();

        assert_eq!(shapes.len(), 3)
    }

    #[test]
    fn shape_already_defined() {
        let line = "
shape main() {}
shape circle(r) {}
shape circle(r) {}
";
        let tokens = lexer::lex(&line.to_owned()).unwrap();
        let program = parser::parse_program(tokens).unwrap();

        match find_shapes(HashMap::new(), &program, true) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn missing_main_shape() {
        let line = "
shape circle(r) {}
";
        let tokens = lexer::lex(&line.to_owned()).unwrap();
        let program = parser::parse_program(tokens).unwrap();

        match find_shapes(HashMap::new(), &program, true) {
            Ok(_) => assert!(false),
            Err(e) => match e.error_type {
                MissingMain => assert!(true),
                _ => assert!(false),
            },
        };
    }

    #[test]
    fn eval_simple_program() {
        let line = "
shape main() {
  svg(value: \"hello\")
}
";
        let value = run_program(line).unwrap();
        assert_debug_snapshot_matches!(value)
    }

    #[test]
    fn eval_program_with_call() {
        let line = "
shape test1(c) {
  svg(value: c)
}

shape test2(c) {
  test1(c: c)
}

shape main() {
  test2(c: \"hello\")
}
";
        let value = run_program(line).unwrap();
        assert_debug_snapshot_matches!(value)
    }

    #[test]
    fn eval_program_with_default_arg() {
        let line = "
shape test(c = \"hello\") {
  svg(value: c)
}

shape main() {
  test()
}
";
        let value = run_program(line).unwrap();
        assert_debug_snapshot_matches!(value)
    }

    #[test]
    fn shape_scope1() {
        let line = "
shape test1(r) {
  svg(value: c)
}

shape test2(c) {
  test1(r: c)
}

shape main() {
  test2(c: \"hello\")
}
";
        match run_program(&line) {
            Ok(_) => assert!(false),
            Err(e) => match e.error_type {
                VariableNotDefined(_) => assert!(true),
                _ => assert!(false),
            },
        };
    }

    #[test]
    fn stack_overflow() {
        let line = "
shape test() {
  test()
}

shape main() {
  test()
}
";

        match run_program(&line) {
            Ok(_) => assert!(false),
            Err(e) => match e.error_type {
                StackOverflow(_) => assert!(true),
                _ => assert!(false),
            },
        };
    }
}
