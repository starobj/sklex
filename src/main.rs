extern crate sklex;

use std::io::{self, Read};
use sklex::{Lexeme, lex};

pub fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut source_code = String::new();

    let _ = stdin.lock().read_to_string(&mut source_code);

    let mut lexer = lex(source_code.as_str());

    println!("|");

    while let Some(token) = lexer.next() {
        println!("{:?}", token);

        match token.lexeme {
            Lexeme::Indent
            | Lexeme::Dedent => {
                println!("{}|", " ".repeat(lexer.indent_size).repeat(lexer.indent_level()))
            },
            _ => {}
        }
    }

    Ok(())
}
