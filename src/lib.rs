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

        assert_eq!(lexer.next(), Some(Token::Text("Create")));

        assert_eq!(lexer.next(), Some(Token::Space(" ")));

        assert_eq!(lexer.next(), Some(Token::Text("ridiculously")));

        assert_eq!(lexer.next(), Some(Token::Space(" ")));

        assert_eq!(lexer.next(), Some(Token::Text("fast")));

        assert_eq!(lexer.next(), Some(Token::Space(" ")));

        assert_eq!(lexer.next(), Some(Token::Text("Lexers")));

        assert_eq!(lexer.next(), Some(Token::Symbol(".")));

        assert_eq!(lexer.next(), None);
    }
}
