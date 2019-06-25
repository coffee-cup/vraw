mod ast;
mod error;

use std::iter::Peekable;
use std::slice::Iter;

use crate::lexer::*;
use crate::utils::*;

use ast::*;
use error::ParseErrorType::*;
use error::*;

const RESERVED: &'static [&'static str] = &["shape"];

pub struct Parser<'a> {
    input: Peekable<Iter<'a, Token>>,
    input_end_pos: Pos,
}

fn is_reserved_word(word: &str) -> bool {
    RESERVED.contains(&word)
}

pub fn parse_program(tokens: Vec<Token>) -> ParseResult<Program> {
    let mut parser = Parser::new(&tokens);
    parser.program()
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
            TokenType::LParen => parser.parse_function_call(lhs),
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
        let mut shapes: Vec<Shape> = vec![];

        while let Some(_) = self.input.peek() {
            let shape = self.shape()?;
            shapes.push(shape);
        }

        Ok(Program { shapes: shapes })
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

        // left curly
        if let None = self.match_next(TokenType::LCurly) {
            return parse_error(Expected("'{' after shape name".to_owned(), None), ident_pos);
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
            None => parse_error(
                Expected(format!("expected shape {} to end with `}}`", ident), None),
                start,
            ),
        }
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
        let t = match self.consume() {
            Some(t) => t,
            // TODO: should use end of input as pos
            None => return parse_error(UnExpectedEndOfInput, self.input_end_pos),
        };

        match t.token_type() {
            TokenType::Ident(s) => {
                if s == word {
                    Ok(t.pos())
                } else {
                    parse_error(Expected(word.to_owned(), Some(s)), t.pos())
                }
            }
            _ => parse_error(Expected("identifier".to_owned(), None), t.pos()),
        }
    }

    pub fn parse_function_call(&mut self, lhs: Expr) -> ParseResult<Expr> {
        let ident = match &lhs {
            Expr::Name(n, _) => n.clone(),
            e => {
                return parse_error(
                    Expected("function name to be an identifier".to_owned(), None),
                    e.pos(),
                )
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
            Some(t) => parse_error(
                Expected(
                    "')' to close function call".to_owned(),
                    Some(format!("{:?}", t)),
                ),
                token.unwrap().token_pos().start,
            ),
            None => parse_error(UnExpectedEndOfInput, lhs.pos()),
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

        assert_eq!(2, program.shapes.len());
        assert_debug_snapshot_matches!(program);
    }
}
