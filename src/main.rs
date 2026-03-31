extern crate sklex;

use std::io::{self, BufRead};
use sklex::{Token, lex};

pub fn main() -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?; // Unwrap or handle the Result
        let mut lexer = lex(line.as_str());

        while let Some(token) = lexer.next() {
            match token {
                Ok(token) => println!("{:?}({:?})", token, lexer.slice()),
                Err(_) => println!("Error"),
            }
        }

        println!("{:?}", Token::Newline);
    }

    Ok(())
}
