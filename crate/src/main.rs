use std::io::{self, Read};

mod error;
mod interpret;
mod lexer;
mod parser;
mod utils;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

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

    let program = match parser::parse_program(tokens) {
        Ok(program) => program,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    let result = match interpret::eval_program(&program) {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    println!("{:?}", result);

    Ok(())
}
