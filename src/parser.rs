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
    pos: Pos,
}

#[derive(Debug, PartialEq)]
pub struct Shape {
    name: Ident,
    args: Vec<Arg>,
    stmts: Vec<Stmt>,
    pos: Pos,
}

#[derive(Debug, PartialEq)]
pub struct NamedArg {
    name: Ident,
    expr: Expr,
    pos: Pos,
}

#[derive(Debug, PartialEq)]
pub struct Stmt {
    expr: Expr,
    pos: Pos,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Name(Ident, Range),
    FunCall(Ident, Vec<NamedArg>, Range),
    Literal(Literal, Range),
    Binary(Box<Expr>, BinOp, Box<Expr>, Pos),
    Unary(UnOp, Box<Expr>, Pos),
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

fn prec(p: Precedence) -> u32 {
    p as u32
}

pub struct Parser<'a> {
    input: Peekable<Iter<'a, Token>>,
}

impl Token {
    // null denotation
    fn nud(&self, parser: &mut Parser) -> Result<Expr, Error> {
        match self.token_type() {
            TokenType::Ident(s) => Ok(Expr::Name(s, self.token_pos())),
            TokenType::Number(n) => Ok(Expr::Literal(Literal::Number(n), self.token_pos())),
            TokenType::Minus => {
                let e = parser.expression(prec(Precedence::Prefix))?;
                Ok(Expr::Unary(UnOp::Neg, Box::new(e), self.token_pos().start))
            }
            TokenType::LParen => {
                let e = parser.expression(0)?;
                if let Some(TokenType::RParen) = parser.input.next().map(|t| t.token_type()) {
                    Ok(e)
                } else {
                    Err(Error::new(
                        "unbalanced paren".to_owned(),
                        Some(self.token_pos().start),
                    ))
                }
            }
            _ => Err(Error::new(
                "expecting literal".to_owned(),
                Some(self.token_pos().start),
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
            _ => Err(Error::new(
                "expecting operator".to_owned(),
                Some(self.token_pos().start),
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

    pub fn expression(&mut self, rbp: u32) -> Result<Expr, Error> {
        let mut left = self.parse_nud()?;
        while self.next_binds_tighter(rbp) {
            left = self.parse_led(left)?;
        }

        Ok(left)
    }

    fn next_binds_tighter(&mut self, rbp: u32) -> bool {
        self.input.peek().map_or(false, |t| t.lbp() > rbp)
    }

    fn parse_nud(&mut self) -> Result<Expr, Error> {
        self.input
            .next()
            .map_or(Err(Error::new("incomplete".to_owned(), None)), |t| {
                t.nud(self)
            })
    }

    fn parse_led(&mut self, expr: Expr) -> Result<Expr, Error> {
        self.input
            .next()
            .map_or(Err(Error::new("incomplete".to_owned(), None)), |t| {
                t.led(self, expr)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use crate::utils::*;

    fn parse_expression(input: &String) -> Result<Expr, Error> {
        let tokens = lexer::lex(input)?;
        let mut parser = Parser::new(tokens.iter());
        parser.expression(0)
    }

    #[test]
    fn parse_number_literal() {
        let ast = parse_expression(&"2".to_owned());
        assert_eq!(
            ast,
            Ok(Expr::Literal(
                Literal::Number(2.0),
                create_range(create_pos(0, 0), create_pos(0, 1))
            ))
        );
    }

    #[test]
    fn parse_simple_identifier() {
        let ast = parse_expression(&"hello".to_owned());
        assert_eq!(
            ast,
            Ok(Expr::Name(
                "hello".to_owned(),
                create_range(create_pos(0, 0), create_pos(0, 5))
            )),
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        let ast = parse_expression(&"10 + 2".to_owned());
        assert_eq!(
            ast,
            Ok(Expr::Binary(
                Box::new(Expr::Literal(
                    Literal::Number(10.0),
                    create_range(create_pos(0, 0), create_pos(0, 2))
                )),
                BinOp::Add,
                Box::new(Expr::Literal(
                    Literal::Number(2.0),
                    create_range(create_pos(0, 5), create_pos(0, 6))
                )),
                create_pos(0, 3)
            ))
        );
    }
}
