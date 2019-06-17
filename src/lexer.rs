use std::iter::Peekable;

use crate::error::*;
use crate::utils::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LParen,
    RParen,
    LCurly,
    RCurly,
    Times,
    Divide,
    Plus,
    Minus,
    Equals,
    Compare,
    Colon,
    Comma,
    Number(f64),
    Ident(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    token_pos: Range,
}

pub struct Lexer<'a> {
    iter: Peekable<std::str::Chars<'a>>,
    line: u32,
    column: u32,
}

pub fn lex(input: &String) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let mut lexer = Lexer::new(input);

    while let Some(t) = lexer.next()? {
        tokens.push(t);
    }

    Ok(tokens)
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn token_pos(&self) -> Range {
        self.token_pos
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        Lexer {
            iter: input.trim().chars().peekable(),
            line: 0,
            column: 0,
        }
    }

    fn pos(&self) -> Pos {
        Pos {
            line: self.line,
            column: self.column,
        }
    }

    fn token(&self, token_type: TokenType, start: Pos, end: Pos) -> Option<Token> {
        Some(Token {
            token_type: token_type,
            token_pos: Range {
                start: start,
                end: end,
            },
        })
    }

    fn advance(&mut self, token_type: TokenType) -> Result<Option<Token>, Error> {
        let start = self.pos();

        self.iter.next();
        self.column += 1;

        let end = self.pos();

        let token = self.token(token_type, start, end);
        Ok(token)
    }

    fn advance_whitespace(&mut self) -> bool {
        match self.iter.peek() {
            Some(' ') | Some('\t') => {
                self.column += 1;
                self.iter.next();
                true
            }
            Some('\n') | Some('\r') => {
                self.column = 0;
                self.line += 1;
                self.iter.next();
                true
            }
            _ => false,
        }
    }

    fn forward(&mut self) -> Option<char> {
        self.column += 1;
        self.iter.next()
    }

    fn match_next(&mut self, pred: &Fn(char) -> bool) -> Option<char> {
        let c = match self.iter.peek() {
            Some(c) => *c,
            None => return None,
        };

        if pred(c) {
            self.forward();
            Some(c)
        } else {
            None
        }
    }

    fn consume_ident(&mut self) -> Result<Option<Token>, Error> {
        let start = self.pos();
        let mut id = String::new();

        match self.match_next(&is_alpha) {
            Some(c) => id.push(c),
            None => {
                return Err(Error::new(
                    "identifiers must start with letter.".to_owned(),
                    Some(self.pos()),
                ))
            }
        };

        while let Some(c) = self.match_next(&is_alphanum) {
            id.push(c);
        }
        let end = self.pos();

        let token = self.token(TokenType::Ident(id), start, end);
        Ok(token)
    }

    fn consume_number(&mut self) -> Result<Option<Token>, Error> {
        let start = self.pos();

        let mut str = String::new();
        let mut seen_dot = false;

        while let Some(c) = self.iter.peek() {
            if *c == '.' && !seen_dot {
                seen_dot = true;
                str.push(self.forward().unwrap());
            } else if is_digit(*c) {
                str.push(self.forward().unwrap());
            } else {
                break;
            }
        }

        let end = self.pos();

        let n = str.parse::<f64>().unwrap();

        Ok(self.token(TokenType::Number(n), start, end))
    }

    fn next(&mut self) -> Result<Option<Token>, Error> {
        // ignore whitespace
        while self.advance_whitespace() {}

        let c = match self.iter.peek() {
            Some(c) => c,
            None => return Ok(None),
        };

        let token = match c {
            '(' => self.advance(TokenType::LParen),
            ')' => self.advance(TokenType::RParen),
            '{' => self.advance(TokenType::LCurly),
            '}' => self.advance(TokenType::RCurly),
            '*' => self.advance(TokenType::Times),
            '/' => self.advance(TokenType::Divide),
            '+' => self.advance(TokenType::Plus),
            '-' => self.advance(TokenType::Minus),
            ':' => self.advance(TokenType::Colon),
            ',' => self.advance(TokenType::Comma),
            '=' => {
                let start = self.pos();
                self.forward();
                let end = self.pos();
                match self.iter.peek() {
                    Some('=') => self.advance(TokenType::Compare),
                    _ => Ok(self.token(TokenType::Equals, start, end)),
                }
            }
            'a'...'z' => self.consume_ident(),
            '0'...'9' => self.consume_number(),
            _ => Err(Error::new(
                format!("unexpected character: {:?}", c),
                Some(self.pos()),
            )),
        };

        token
    }
}

fn is_alpha(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' => true,
        _ => false,
    }
}

fn is_digit(c: char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}

fn is_alphanum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_identifier() {
        let tokens = lex(&"a1234".to_owned());
        assert_eq!(
            tokens,
            Ok(vec![Token {
                token_type: TokenType::Ident("a1234".to_owned()),
                token_pos: Range {
                    start: Pos { line: 0, column: 0 },
                    end: Pos { line: 0, column: 5 }
                }
            }])
        );
    }

    #[test]
    fn lex_integer() {
        let tokens = lex(&"10".to_owned());
        assert_eq!(
            tokens,
            Ok(vec![Token {
                token_type: TokenType::Number(10.0),
                token_pos: Range {
                    start: Pos { line: 0, column: 0 },
                    end: Pos { line: 0, column: 2 }
                }
            }])
        )
    }

    #[test]
    fn lex_float() {
        let tokens = lex(&"10.123".to_owned());
        assert_eq!(
            tokens,
            Ok(vec![Token {
                token_type: TokenType::Number(10.123),
                token_pos: Range {
                    start: Pos { line: 0, column: 0 },
                    end: Pos { line: 0, column: 6 }
                }
            }])
        )
    }
}
