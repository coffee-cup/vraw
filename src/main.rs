use std::io::{self, Read};

mod error;
mod lexer;
mod parser;
mod utils;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let tokens = match lexer::lex(&buffer) {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    let mut parser = parser::Parser::new(tokens.iter());
    let ast = parser.expression(0);

    println!("{:?}", ast);

    Ok(())
}
