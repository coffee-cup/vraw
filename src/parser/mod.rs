use std::iter::Peekable;
use std::slice::Iter;

use crate::lexer::*;
use crate::utils::*;

pub mod ast;
mod error;

use ast::*;
use error::ParseErrorType::*;
use error::*;

const RESERVED: &'static [&'static str] = &["shape", "transform"];

struct Parser<'a> {
    input: Peekable<Iter<'a, Token>>,
    input_end_pos: Pos,
}

type ParseResult<T> = Result<T, ParseError>;

fn parse_error<T>(error_type: ParseErrorType, pos: Pos) -> ParseResult<T> {
    Err(ParseError {
        error_type: error_type,
        pos: pos,
    })
}

fn is_reserved_word(word: &str) -> bool {
    RESERVED.contains(&word)
}

pub fn parse_program(tokens: Vec<Token>) -> ParseResult<Program> {
    let mut parser = Parser::new(&tokens);
    parser.program()
}

pub fn parse_expression(tokens: Vec<Token>) -> ParseResult<Expr> {
    let mut parser = Parser::new(&tokens);
    parser.expression(0)
}

impl Token {
    // null denotation
    fn nud(&self, parser: &mut Parser) -> ParseResult<Expr> {
        match self.token_type() {
            TokenType::Ident(s) => {
                if is_reserved_word(s.as_str()) {
                    parse_error(IdentiferCannotBeReservedWord(s), self.token_pos().start)
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
                    parse_error(UnBalancedParen, self.token_pos().start)
                }
            }
            t => parse_error(
                Expected("literal".to_owned(), Some(format!("{:?}", t))),
                self.token_pos().start,
            ),
        }
    }

    // left denotation
    fn led(&self, parser: &mut Parser, lhs: Expr) -> ParseResult<Expr> {
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
            t => parse_error(
                Expected("operator".to_owned(), Some(format!("{:?}", t))),
                self.token_pos().start,
            ),
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
    pub fn new(input: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            input: input.iter().peekable(),
            input_end_pos: input[input.len() - 1].token_pos().end,
        }
    }

    pub fn statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression(0)?;
        let pos = expr.pos();

        Ok(Stmt::Expr(expr, pos))
    }

    pub fn expression(&mut self, rbp: u32) -> ParseResult<Expr> {
        let mut left = self.parse_nud()?;
        while self.next_binds_tighter(rbp) {
            left = self.parse_led(left)?;
        }

        Ok(left)
    }

    pub fn program(&mut self) -> ParseResult<Program> {
        let mut decls: Vec<Decl> = vec![];

        while let Some(_) = self.input.peek() {
            let decl = self.decl()?;
            decls.push(decl);
        }

        Ok(Program {
            decls: decls,
            end: self.input_end_pos,
        })
    }

    pub fn decl(&mut self) -> ParseResult<Decl> {
        let shape = self.shape()?;
        Ok(Decl::ShapeDecl(shape))
    }

    pub fn shape(&mut self) -> ParseResult<Shape> {
        // word shape
        let start = self.parse_reserved_word("shape")?;

        // shape name
        let (ident, ident_pos) = self.parse_ident()?;

        // left paren
        if let None = self.match_next(TokenType::LParen) {
            return parse_error(Expected("'(' after shape name".to_owned(), None), ident_pos);
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
            Some(t) => {
                return parse_error(
                    Expected(
                        "expecting ')' to close function call.".to_owned(),
                        Some(format!("{:?}", t)),
                    ),
                    token.unwrap().token_pos().start,
                )
            }
            None => return parse_error(UnExpectedEndOfInput, self.input_end_pos),
        };

        // block
        let block = self.parse_block()?;

        Ok(Shape {
            name: ident,
            args: args,
            block: block,
            pos: start,
        })
    }

    fn next_binds_tighter(&mut self, rbp: u32) -> bool {
        self.input.peek().map_or(false, |t| t.lbp() > rbp)
    }

    pub fn parse_ident(&mut self) -> ParseResult<(Ident, Pos)> {
        let expr = self.parse_nud()?;
        match &expr {
            Expr::Name(n, _) => {
                if is_reserved_word(&n.as_str()) {
                    parse_error(IdentiferCannotBeReservedWord(n.to_string()), expr.pos())
                } else {
                    Ok((n.clone(), expr.pos()))
                }
            }
            e => parse_error(Expected("identifier".to_owned(), None), e.pos()),
        }
    }

    pub fn parse_reserved_word(&mut self, word: &str) -> ParseResult<Pos> {
        let token = match self.consume() {
            Some(t) => t,
            None => return parse_error(UnExpectedEndOfInput, self.input_end_pos),
        };

        match token.token_type() {
            TokenType::Ident(s) => {
                if s == word {
                    Ok(token.pos())
                } else {
                    parse_error(Expected(word.to_owned(), Some(s)), token.pos())
                }
            }
            _ => parse_error(Expected("identifier".to_owned(), None), token.pos()),
        }
    }

    pub fn parse_function_call(&mut self) -> ParseResult<FunCall> {
        let (ident, ident_pos) = self.parse_ident()?;

        // left paren
        let token = self.input.next();
        match token.map(|t| t.token_type()) {
            Some(TokenType::LParen) => (),
            Some(_) => {
                return parse_error(
                    Expected("'(' to after function name".to_owned(), None),
                    token.unwrap().token_pos().start,
                )
            }
            None => return parse_error(UnExpectedEndOfInput, self.input_end_pos),
        }

        // args
        let mut args: Vec<NamedArg> = vec![];
        if self.next_token_type() != Some(TokenType::RParen) {
            let mut arg = self.parse_named_arg()?;
            args.push(arg);

            while let Some(_) = self.match_next(TokenType::Comma) {
                arg = self.parse_named_arg()?;
                args.push(arg);
            }
        }

        // right paren
        let token = self.input.next();
        match token.map(|t| t.token_type()) {
            Some(TokenType::RParen) => (),
            Some(t) => {
                return parse_error(
                    Expected(
                        "')' to close function call".to_owned(),
                        Some(format!("{:?}", t)),
                    ),
                    token.unwrap().token_pos().start,
                )
            }
            None => return parse_error(UnExpectedEndOfInput, ident_pos),
        }

        Ok(FunCall {
            ident: ident,
            args: args,
            range: create_range(ident_pos, token.unwrap().token_pos().start),
        })
    }

    pub fn parse_block(&mut self) -> ParseResult<Block> {
        // left curly
        let token = self.input.next();
        match token.map(|t| t.token_type()) {
            Some(TokenType::LCurly) => (),
            Some(_) => {
                return parse_error(
                    Expected("'{' to begin block".to_owned(), None),
                    token.unwrap().token_pos().start,
                )
            }
            None => return parse_error(UnExpectedEndOfInput, self.input_end_pos),
        }

        let mut calls: Vec<FunCall> = vec![];

        let mut last = self.match_next(TokenType::RCurly);
        while let None = last {
            let call = self.parse_function_call()?;
            calls.push(call);

            last = self.match_next(TokenType::RCurly);
        }

        match &last {
            Some(_) => Ok(Block {
                calls: calls,
                range: create_range(
                    token.unwrap().token_pos().start,
                    last.unwrap().token_pos().end,
                ),
            }),
            None => parse_error(
                Expected("block to end with '}}'".to_owned(), None),
                token.unwrap().token_pos().start,
            ),
        }
    }

    pub fn parse_named_arg(&mut self) -> ParseResult<NamedArg> {
        let (name, _) = self.parse_ident()?;
        self.match_next(TokenType::Colon);

        let e = match self.expression(0) {
            Err(err) => {
                return parse_error(
                    Expected(
                        "parameters to functions to be in format `(name: value)`".to_owned(),
                        None,
                    ),
                    err.pos(),
                )
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

    fn parse_nud(&mut self) -> ParseResult<Expr> {
        self.input
            .next()
            .map_or(parse_error(UnExpectedEndOfInput, self.input_end_pos), |t| {
                t.nud(self)
            })
    }

    fn parse_led(&mut self, expr: Expr) -> ParseResult<Expr> {
        self.input
            .next()
            .map_or(parse_error(UnExpectedEndOfInput, self.input_end_pos), |t| {
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

    fn parse_expression(input: &String) -> ParseResult<Expr> {
        let tokens = lexer::lex(input).unwrap();
        let mut parser = Parser::new(&tokens);
        parser.expression(0)
    }

    fn parse_shape(input: &String) -> ParseResult<Shape> {
        let tokens = lexer::lex(input).unwrap();
        let mut parser = Parser::new(&tokens);
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

        assert_eq!(2, program.decls.len());
        assert_debug_snapshot_matches!(program);
    }
}
