pub mod lexical_analysis;
pub mod token;

pub use lexical_analysis::*;
pub use token::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lexer = lex("Create ridiculously fast Lexers.");

        assert_eq!(lexer.next(), Some(Token::new(Lexeme::Text("Create"), 0..6)));
        assert_eq!(lexer.next(), Some(Token::new(Lexeme::Space(" "), 6..7)));
        assert_eq!(lexer.next(), Some(Token::new(Lexeme::Text("ridiculously"), 7..19)));
        assert_eq!(lexer.next(), Some(Token::new(Lexeme::Space(" "), 19..20)));
        assert_eq!(lexer.next(), Some(Token::new(Lexeme::Text("fast"), 20..24)));
        assert_eq!(lexer.next(), Some(Token::new(Lexeme::Space(" "), 24..25)));
        assert_eq!(lexer.next(), Some(Token::new(Lexeme::Text("Lexers"), 25..31)));
        assert_eq!(lexer.next(), Some(Token::new(Lexeme::Symbol("."), 31..32)));

        assert_eq!(lexer.next(), None);
    }
}
