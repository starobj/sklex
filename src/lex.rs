use std::ops::Range;

use logos::{Lexer, Logos};

use crate::Token;

#[derive(Debug, PartialEq)]
pub enum IndentAwareToken {
    Token(Token),
    Indent(usize),  // Now stores the indentation level (e.g., 1, 2, 3, ...)
    Dedent(usize),  // Now stores the indentation level (e.g., 1, 2, 3, ...)
}

pub struct IndentAwareLexer<'a> {
    inner: Lexer<'a, Token>,
    indent_stack: Vec<usize>,  // Tracks indentation levels in spaces
    pending_newline: bool,
    next_token: Option<Result<Token, ()>>,
    indent_size: usize,  // Number of spaces per indentation level (e.g., 4)
}

impl<'a> IndentAwareLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Token::lexer(source);
        let next_token = lexer.next();
        Self {
            inner: lexer,
            indent_stack: vec![0],
            pending_newline: false,
            next_token,
            indent_size: 4,  // Default: 4 spaces per indentation level
        }
    }

    pub fn peek(&mut self) -> Option<&Result<Token, ()>> {
        self.next_token.as_ref()
    }

    pub fn slice(&mut self) -> &str {
        self.inner.slice()
    }

    pub fn span(&mut self) -> Range<usize> {
        self.inner.span()
    }
}

impl<'a> Iterator for IndentAwareLexer<'a> {
    type Item = IndentAwareToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pending_newline {
            self.pending_newline = false;
            if let Some(Ok(Token::Space)) = self.peek() {
                // Consume the Space token
                let _ = self.next_token.take();
                let space_len = self.inner.slice().len();
                let current_indent = *self.indent_stack.last().unwrap();

                if space_len > current_indent {
                    // Calculate the new indentation level
                    let new_level = space_len / self.indent_size;
                    self.indent_stack.push(space_len);
                    return Some(IndentAwareToken::Indent(new_level));  // Use new_level here
                } else if space_len < current_indent {
                    // Calculate how many levels to dedent
                    while *self.indent_stack.last().unwrap() > space_len {
                        let prev_indent = self.indent_stack.pop().unwrap();
                        let dedent_level = prev_indent / self.indent_size;
                        return Some(IndentAwareToken::Dedent(dedent_level));
                    }
                }
                // If equal, just skip the space
                self.next_token = self.inner.next();
                return self.next();
            }
        }

        match self.next_token.take() {
            Some(Ok(token)) => {
                self.next_token = self.inner.next();
                if matches!(token, Token::Block | Token::Newline) {
                    self.pending_newline = true;
                }
                Some(IndentAwareToken::Token(token))
            }
            Some(Err(_)) => None,
            None => None,
        }
    }
}

pub fn lex<'a>(source: &'a str) -> IndentAwareLexer<'a> {
    IndentAwareLexer::new(source)
}
