extern crate sklex;

use std::io::{self, BufRead};
use sklex::{Token, lex};

pub fn main() -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?; // Unwrap or handle the Result
        let mut lexer = lex(line.as_str());

        while let Some(token) = lexer.next() {
            let lexeme = match token {
                // Match any variant that contains a lexeme
                Token::Block(l)
                | Token::Newline(l)
                | Token::Space(l)
                | Token::Number(l)
                | Token::Integer(l)
                | Token::String(l)
                | Token::XMLTagOpen(l)
                | Token::XMLTagClose(l)
                | Token::Text(l)
                | Token::Symbol(l)
                | Token::Indent(l)
                | Token::Dedent(l) => l,
                // Handle the Error variant separately
                Token::Error => "(error)",
            };
            println!("{:?}({:?})", token, lexeme);
        }

        println!("{:?}", Token::Newline("\n"));
    }

    Ok(())
}
