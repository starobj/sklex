pub mod lexical_analysis;
pub mod token;

pub use lexical_analysis::*;
pub use token::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_invalid() {
        // This test is a placeholder; no known invalid sequences exist.
        // If you find one, please create an issue and I'll implement the test.
    }

    #[test]
    fn lex_valid() {
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

    #[test]
    fn lexer_eq() {
        let source: &str = "toadstool goblin";

        let mut lexer_a = lex(source);
        let mut lexer_b = lex(source);

        assert_eq!(lexer_a, lexer_b);

        assert_eq!(lexer_a.next(), Some(Token::new(Lexeme::Text("toadstool"), 0..9)));

        assert_ne!(lexer_a, lexer_b);

        assert_eq!(lexer_b.next(), Some(Token::new(Lexeme::Text("toadstool"), 0..9)));

        assert_eq!(lexer_a, lexer_b);

        assert_eq!(lexer_a.next(), Some(Token::new(Lexeme::Space(" "), 9..10)));

        assert_ne!(lexer_a, lexer_b);

        assert_eq!(lexer_b.next(), Some(Token::new(Lexeme::Space(" "), 9..10)));

        assert_eq!(lexer_a, lexer_b);

        assert_eq!(lexer_a.next(), Some(Token::new(Lexeme::Text("goblin"), 10..16)));

        assert_ne!(lexer_a, lexer_b);

        assert_eq!(lexer_b.next(), Some(Token::new(Lexeme::Text("goblin"), 10..16)));

        assert_eq!(lexer_a, lexer_b);

        assert_eq!(lexer_a.next(), None);

        assert_ne!(lexer_a, lexer_b);

        assert_eq!(lexer_b.next(), None);

        assert_eq!(lexer_a, lexer_b);
    }
}
