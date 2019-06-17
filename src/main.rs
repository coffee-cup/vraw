use std::io::{self, Read};

mod error;
mod lexer;
mod utils;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    match lexer::lex(&buffer) {
        Ok(tokens) => println!("{:?}", tokens),
        Err(err) => println!("{}", err),
    }

    Ok(())
}
