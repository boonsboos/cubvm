use std::fs;
use std::process::exit;

use super::codegen::{self, Code};

use super::token;

pub fn compile(path: String) -> Code {

    let source_r = fs::read_to_string(path.clone());

    let source = match source_r {
        Ok(o) => o,
        Err(_) => panic!("failed to open file {}", path),
    };

    let lines: Vec<String> = source.lines().map(String::from).collect();

    let tokens: Vec<token::TokenKind> = token::tokenize(lines);

    match codegen::generate(tokens) {
        Ok(o) => return o,
        Err(_) => exit(1), // generate will print its errors
    }
}