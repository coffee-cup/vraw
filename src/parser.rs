use std::iter::Peekable;
use std::slice::Iter;

use crate::error::*;
use crate::lexer::*;
use crate::utils::*;

type Ident = String;
type Arg = Ident;

#[derive(Debug, PartialEq)]
pub struct Program {
    shapes: Vec<Shape>,
}

#[derive(Debug, PartialEq)]
pub struct Shape {
    name: Ident,
    args: Vec<Arg>,
    stmts: Vec<Stmt>,
    range: Range,
}

#[derive(Debug, PartialEq)]
pub struct NamedArg {
    name: Ident,
    expr: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr, Pos),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Name(Ident, Range),
    FunCall(Ident, Vec<NamedArg>, Range),
    Literal(Literal, Range),
    Binary(Box<Expr>, BinOp, Box<Expr>, Pos),
    Unary(UnOp, Box<Expr>, Pos),
    Grouping(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    Neg,
}

#[derive(Copy, Clone)]
enum Precedence {
    Conditional = 20,
    Sum = 30,
    Product = 40,
    Exponent = 50,
    Prefix = 60,
    Postfix = 70,
    Call = 80,
}

const RESERVED: &'static [&'static str] = &["shape"];

impl HasPos for Expr {
    fn pos(&self) -> Pos {
        match self {
            Expr::Name(_, range) => range.start,
            Expr::FunCall(_, _, range) => range.start,
            Expr::Literal(_, range) => range.start,
            Expr::Binary(_, _, _, p) => *p,
            Expr::Unary(_, _, p) => *p,
            Expr::Grouping(ref e) => e.pos(),
        }
    }
}

impl HasPos for Stmt {
    fn pos(&self) -> Pos {
        match self {
            Stmt::Expr(_, p) => *p,
        }
    }
}

fn prec(p: Precedence) -> u32 {
    p as u32
}

pub struct Parser<'a> {
    input: Peekable<Iter<'a, Token>>,
}

fn is_reserved_word(word: &str) -> bool {
    RESERVED.contains(&word)
}

pub fn parse_program(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut parser = Parser::new(tokens.iter());
    parser.program()
}

impl Token {
    // null denotation
    fn nud(&self, parser: &mut Parser) -> Result<Expr, Error> {
        match self.token_type() {
            TokenType::Ident(s) => {
                if is_reserved_word(s.as_str()) {
                    Err(Error::new(
                        format!("identifier `{}` is a reserved word", s),
                        self.token_pos().start,
                    ))
                } else {
                    Ok(Expr::Name(s, self.token_pos()))
                }
            }
            TokenType::Number(n) => Ok(Expr::Literal(Literal::Number(n), self.token_pos())),
            TokenType::Minus => {
                let e = parser.expression(prec(Precedence::Prefix))?;
                Ok(Expr::Unary(UnOp::Neg, Box::new(e), self.token_pos().start))
            }
            TokenType::LParen => {
                let e = parser.expression(0)?;
                if let Some(TokenType::RParen) = parser.input.next().map(|t| t.token_type()) {
                    Ok(Expr::Grouping(Box::new(e)))
                } else {
                    Err(Error::new(
                        "unbalanced paren".to_owned(),
                        self.token_pos().start,
                    ))
                }
            }
            t => Err(Error::new(
                format!("expecting literal. received {:?}", t),
                self.token_pos().start,
            )),
        }
    }

    // left denotation
    fn led(&self, parser: &mut Parser, lhs: Expr) -> Result<Expr, Error> {
        match self.token_type() {
            TokenType::Times | TokenType::Divide | TokenType::Plus | TokenType::Minus => {
                let rhs = parser.expression(self.lbp())?;
                let op = token_to_binop(self.clone()).unwrap();
                Ok(Expr::Binary(
                    Box::new(lhs),
                    op,
                    Box::new(rhs),
                    self.token_pos().start,
                ))
            }
            TokenType::LParen => parser.parse_function_call(lhs),
            _ => Err(Error::new(
                "expecting operator".to_owned(),
                self.token_pos().start,
            )),
        }
    }

    // left binding power
    fn lbp(&self) -> u32 {
        match self.token_type() {
            TokenType::LParen => prec(Precedence::Call),
            TokenType::Times | TokenType::Divide => prec(Precedence::Product),
            TokenType::Plus | TokenType::Minus => prec(Precedence::Sum),
            _ => 0,
        }
    }
}

fn token_to_binop(token: Token) -> Result<BinOp, String> {
    match token.token_type() {
        TokenType::Times => Ok(BinOp::Mul),
        TokenType::Divide => Ok(BinOp::Div),
        TokenType::Plus => Ok(BinOp::Add),
        TokenType::Minus => Ok(BinOp::Sub),
        _ => Err("no binop for token".to_owned()),
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: Iter<'a, Token>) -> Parser<'a> {
        Parser {
            input: input.peekable(),
        }
    }

    pub fn statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression(0)?;
        let pos = expr.pos();

        Ok(Stmt::Expr(expr, pos))
    }

    pub fn expression(&mut self, rbp: u32) -> Result<Expr, Error> {
        let mut left = self.parse_nud()?;
        while self.next_binds_tighter(rbp) {
            left = self.parse_led(left)?;
        }

        Ok(left)
    }

    pub fn program(&mut self) -> Result<Program, Error> {
        let mut shapes: Vec<Shape> = vec![];

        while let Some(_) = self.input.peek() {
            let shape = self.shape()?;
            shapes.push(shape);
        }

        Ok(Program { shapes: shapes })
    }

    pub fn shape(&mut self) -> Result<Shape, Error> {
        // word shape
        let start = self.parse_reserved_word("shape")?;

        // shape name
        let (ident, ident_pos) = self.parse_ident()?;

        // left paren
        if let None = self.match_next(TokenType::LParen) {
            return Err(Error::new(
                "expected '(' after shape name".to_owned(),
                ident_pos,
            ));
        }

        // args list
        let mut args: Vec<Ident> = vec![];

        if self.next_token_type() != Some(TokenType::RParen) {
            let (arg, _) = self.parse_ident()?;
            args.push(arg);

            while let Some(_) = self.match_next(TokenType::Comma) {
                let (arg, _) = self.parse_ident()?;
                args.push(arg);
            }
        }

        let token = self.input.next();

        // right paren
        match token.map(|t| t.token_type()) {
            Some(TokenType::RParen) => (),
            Some(_) => {
                return Err(Error::new(
                    "expecting ')' to close function call.".to_owned(),
                    token.unwrap().token_pos().start,
                ))
            }
            None => {
                return Err(Error::new(
                    "unexpected end of input.".to_owned(),
                    // TODO: better pos (should be last pos in stream)
                    create_pos(0, 0),
                ));
            }
        };

        // left curly
        if let None = self.match_next(TokenType::LCurly) {
            return Err(Error::new(
                "expected '{' after shape name".to_owned(),
                ident_pos,
            ));
        }

        // statment list
        let mut stmts: Vec<Stmt> = vec![];

        let mut last = self.match_next(TokenType::RCurly);
        while let None = last {
            let stmt = self.statement()?;
            stmts.push(stmt);

            last = self.match_next(TokenType::RCurly);
        }

        match last {
            Some(t) => Ok(Shape {
                name: ident,
                args: args,
                stmts: stmts,
                range: create_range(start, t.pos()),
            }),
            None => Err(Error::new(
                format!("expected shape {} to end with `}}`", ident),
                start,
            )),
        }
    }

    fn next_binds_tighter(&mut self, rbp: u32) -> bool {
        self.input.peek().map_or(false, |t| t.lbp() > rbp)
    }

    pub fn parse_ident(&mut self) -> Result<(Ident, Pos), Error> {
        let expr = self.parse_nud()?;
        match &expr {
            Expr::Name(n, _) => {
                if is_reserved_word(&n.as_str()) {
                    Err(Error::new(
                        "identifier cannot be a reserved word".to_owned(),
                        expr.pos(),
                    ))
                } else {
                    Ok((n.clone(), expr.pos()))
                }
            }
            e => Err(Error::new("expecting identifier".to_owned(), e.pos())),
        }
    }

    pub fn parse_reserved_word(&mut self, word: &str) -> Result<Pos, Error> {
        let t = match self.consume() {
            Some(t) => t,
            // TODO: should use end of input as pos
            None => {
                return Err(Error::new(
                    "unexpected end of input".to_owned(),
                    create_pos(0, 0),
                ))
            }
        };

        match t.token_type() {
            TokenType::Ident(s) => {
                if s == word {
                    Ok(t.pos())
                } else {
                    Err(Error::new(
                        format!("expected {}. found: {}", word, s),
                        t.pos(),
                    ))
                }
            }
            _ => Err(Error::new("expecting identifier".to_owned(), t.pos())),
        }
    }

    pub fn parse_function_call(&mut self, lhs: Expr) -> Result<Expr, Error> {
        let ident = match &lhs {
            Expr::Name(n, _) => n.clone(),
            e => {
                return Err(Error::new(
                    "function name needs to be an identifier.".to_owned(),
                    e.pos(),
                ))
            }
        };
        let mut args: Vec<NamedArg> = vec![];

        if self.next_token_type() != Some(TokenType::RParen) {
            // we need to capture all the args
            let mut arg = self.parse_named_arg()?;
            args.push(arg);

            while let Some(_) = self.match_next(TokenType::Comma) {
                arg = self.parse_named_arg()?;
                args.push(arg);
            }
        }

        let token = self.input.next();

        match token.map(|t| t.token_type()) {
            Some(TokenType::RParen) => Ok(Expr::FunCall(
                ident,
                args,
                create_range(lhs.pos(), token.unwrap().token_pos().start),
            )),
            Some(_) => Err(Error::new(
                "expecting ')' to close function call.".to_owned(),
                token.unwrap().token_pos().start,
            )),
            None => Err(Error::new("unexpected end of input.".to_owned(), lhs.pos())),
        }
    }

    pub fn parse_named_arg(&mut self) -> Result<NamedArg, Error> {
        let (name, _) = self.parse_ident()?;
        self.match_next(TokenType::Colon);

        let e = match self.expression(0) {
            Err(err) => {
                return Err(Error::new(
                    "parameters to functions must be in format `(name: value)`".to_owned(),
                    err.pos(),
                ))
            }
            Ok(expr) => expr,
        };

        Ok(NamedArg {
            name: name,
            expr: e,
        })
    }

    pub fn match_next(&mut self, token_type: TokenType) -> Option<Token> {
        let t = match &self.input.peek() {
            Some(t) => **t,
            None => return None,
        };

        if t.token_type() == token_type {
            self.consume();
            Some(t.clone())
        } else {
            None
        }
    }

    pub fn next_token_type(&mut self) -> Option<TokenType> {
        self.input.peek().map(|t| t.token_type())
    }

    fn consume(&mut self) -> Option<Token> {
        self.input.next().map(|t| t.clone())
    }

    fn parse_nud(&mut self) -> Result<Expr, Error> {
        self.input.next().map_or(
            // TODO: better position here
            Err(Error::new("incomplete".to_owned(), create_pos(0, 0))),
            |t| t.nud(self),
        )
    }

    fn parse_led(&mut self, expr: Expr) -> Result<Expr, Error> {
        self.input
            .next()
            .map_or(Err(Error::new("incomplete".to_owned(), expr.pos())), |t| {
                t.led(self, expr)
            })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot_matches;

    use super::*;
    use crate::lexer;
    use crate::utils::*;

    fn parse_expression(input: &String) -> Result<Expr, Error> {
        let tokens = lexer::lex(input)?;
        let mut parser = Parser::new(tokens.iter());
        parser.expression(0)
    }

    fn parse_shape(input: &String) -> Result<Shape, Error> {
        let tokens = lexer::lex(input)?;
        let mut parser = Parser::new(tokens.iter());
        parser.shape()
    }

    #[test]
    fn parse_number_literal() {
        let ast = parse_expression(&"2".to_owned());
        assert_debug_snapshot_matches!(ast);
    }

    #[test]
    fn parse_simple_identifier() {
        let ast = parse_expression(&"hello".to_owned());
        assert_debug_snapshot_matches!(ast);
    }

    #[test]
    fn parse_reserved_identifier() {
        let ast = parse_expression(&"shape".to_owned());

        match ast {
            Ok(_) => assert!(false, "reserved word should not be parsable as identifier"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn parse_simple_binary_expression() {
        let ast = parse_expression(&"10 + 2".to_owned());
        assert_debug_snapshot_matches!(ast);
    }

    #[test]
    fn parse_simple_shape() {
        let code = "shape circle(r) {
  ellipse(rx: r, ry: r)
}";

        let ast = parse_shape(&code.to_owned());
        assert_debug_snapshot_matches!(ast);
    }

    #[test]
    fn parse_shape_multiple_statements() {
        let code = "shape circle(r) {
  ellipse(rx: r, ry: r * 10)
  ellipse(rx: r, ry: r)
  ellipse(rx: r, ry: r * 10)
  ellipse(rx: r, ry: r * 10)
  ellipse(rx: r, ry: r)
  ellipse(rx: r, ry: r)
}";

        let ast = parse_shape(&code.to_owned());
        assert_debug_snapshot_matches!(ast);
    }

    #[test]
    fn parse_simple_program() {
        let code = "
shape shape1(r) {
  circle(r: 10)
}

shape shape2(r) {
  circle(r: 20)
}
";

        let tokens = lexer::lex(&code.to_owned()).unwrap();
        let program = parse_program(tokens).unwrap();

        assert_eq!(2, program.shapes.len());
        assert_debug_snapshot_matches!(program);
    }
}
