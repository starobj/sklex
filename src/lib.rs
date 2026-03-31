pub mod lex;
pub mod token;

pub use lex::*;
pub use token::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lexer = lex("Create ridiculously fast Lexers.");

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.span(), 0..6);
        assert_eq!(lexer.slice(), "Create");

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.span(), 7..19);
        assert_eq!(lexer.slice(), "ridiculously");

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.span(), 20..24);
        assert_eq!(lexer.slice(), "fast");

        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        assert_eq!(lexer.slice(), "Lexers");
        assert_eq!(lexer.span(), 25..31);

        assert_eq!(lexer.next(), Some(Ok(Token::Symbol)));
        assert_eq!(lexer.span(), 31..32);
        assert_eq!(lexer.slice(), ".");

        assert_eq!(lexer.next(), None);
    }
}
