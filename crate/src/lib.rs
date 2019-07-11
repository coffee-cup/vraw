use std::fmt;
use wasm_bindgen::prelude::*;

mod error;
mod interpret;
mod lexer;
mod parser;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone)]
pub struct CompileError {
    pub line: u32,
    pub column: u32,
    message: String,
}

#[wasm_bindgen]
impl CompileError {
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CompileResult {
    svg: Option<String>,
    error: Option<CompileError>,
}

#[wasm_bindgen]
impl CompileResult {
    pub fn get_svg(&self) -> Option<String> {
        match &self.svg {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }

    pub fn get_error(&self) -> Option<CompileError> {
        match &self.error {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }
}

#[wasm_bindgen]
pub struct Foo {
    test: String,
}

#[wasm_bindgen]
impl Foo {
    pub fn go_riders(&self, x: &str) -> String {
        let mut s = self.test.clone();
        s.push_str(x);
        s
    }
}

#[wasm_bindgen]
pub fn bar(x: &str) -> Foo {
    let foo = Foo {
        test: "hello world".to_owned(),
    };
    foo
}

fn error_to_compile_result<T: fmt::Display>(err: error::Error<T>) -> CompileResult {
    let compile_error = CompileError {
        line: err.pos.line,
        column: err.pos.column,
        message: format!("{}", err.error_type),
    };

    CompileResult {
        svg: None,
        error: Some(compile_error),
    }
}

#[wasm_bindgen]
pub fn compile(input: &str) -> CompileResult {
    let tokens = match lexer::lex(&input.to_owned()) {
        Ok(tokens) => tokens,
        Err(err) => return error_to_compile_result(err),
    };

    let program = match parser::parse_program(tokens) {
        Ok(program) => program,
        Err(err) => return error_to_compile_result(err),
    };

    let result = match interpret::eval_program(&program) {
        Ok(value) => value,
        Err(err) => return error_to_compile_result(err),
    };

    CompileResult {
        svg: Some(result),
        error: None,
    }
}
