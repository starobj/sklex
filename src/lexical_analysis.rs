use logos::{Logos};

use crate::{Lexeme, Token};

pub struct IndentAwareLexer<'a> {
    inner: logos::Lexer<'a, Lexeme<'a>>,
    indent_stack: Vec<usize>,
    pending_newline: bool,
    pending_indent_count: usize,
    pending_dedent_count: usize,
    pub indent_size: usize,
}

impl<'a> IndentAwareLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            inner: Lexeme::lexer(source),
            indent_stack: vec![0],
            pending_newline: false,
            pending_indent_count: 0,
            pending_dedent_count: 0,
            indent_size: 4,
        }
    }

    pub fn indent_level(&self) -> usize {
        return self.indent_stack.len() - 1;
    }
}

impl<'a> Iterator for IndentAwareLexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Peek at the next token without consuming it
        let mut peeker = self.inner.clone();
        if self.pending_indent_count > 0 {
            let current_indent_length = *self.indent_stack.last().unwrap();
            if let Some(Ok(Lexeme::Space(lexeme))) = peeker.next() {
                let span = peeker.span();
                let space_length = lexeme.len();
                let new_level = space_length / self.indent_size;
                let old_level = current_indent_length / self.indent_size;
                self.indent_stack.push(space_length);
                self.pending_indent_count = new_level - old_level - 1;
                // Consume the space.
                self.inner.next();
                return Some(
                    Token {
                        lexeme: Lexeme::Indent,
                        span: span.clone(),
                    }
                );
            }
            else {
                return Some(Token::new(Lexeme::Error, self.inner.span()));
            }
        }
        if self.pending_dedent_count > 0 {
            let current_indent_length = *self.indent_stack.last().unwrap();
            if let Some(Ok(Lexeme::Space(lexeme))) = peeker.next() {
                let span = peeker.span();
                let space_length = lexeme.len();
                let new_level = space_length / self.indent_size;
                let old_level = current_indent_length / self.indent_size;
                self.indent_stack.pop();
                self.pending_indent_count = new_level - old_level - 1;
                // Consume the space.
                self.inner.next();
                return Some(
                    Token {
                        lexeme: Lexeme::Dedent,
                        span,
                    }
                );
            }
            else {
                return Some(Token::new(Lexeme::Error, self.inner.span()));
            }
        }
        if self.pending_newline {
            self.pending_newline = false;
            if let Some(Ok(Lexeme::Space(lexeme))) = peeker.next() {
                let span = peeker.span();
                let space_length = lexeme.len();
                let current_indent_length = *self.indent_stack.last().unwrap();

                if space_length > current_indent_length {
                    let new_level = space_length / self.indent_size;
                    let old_level = current_indent_length / self.indent_size;
                    self.indent_stack.push(space_length);
                    self.pending_indent_count = new_level - old_level - 1;
                    // Consume the space.
                    self.inner.next();
                    return Some(
                        Token {
                            lexeme: Lexeme::Indent,
                            span: span.clone(),
                        }
                    );
                } else if space_length < current_indent_length {
                    let new_level = space_length / self.indent_size;
                    let old_level = current_indent_length / self.indent_size;
                    self.pending_dedent_count = old_level - new_level - 1;
                    self.indent_stack.pop();
                    // Consume the space.
                    self.inner.next();
                    return Some(
                        Token {
                            lexeme: Lexeme::Dedent,
                            span: span.clone(),
                        }
                    );
                }
                // Consume the space.
                self.inner.next();
                return self.next();
            }
            else if self.indent_stack.len() > 1 {
                self.pending_newline = true;
                self.indent_stack.pop();
                let mut span = peeker.span().clone();
                span.end = span.start;
                return Some(
                    Token {
                        lexeme: Lexeme::Dedent,
                        span,
                    }
                );
            }
        }

        match self.inner.next() {
            Some(Ok(lexeme)) => {
                let span = self.inner.span();
                if matches!(lexeme, Lexeme::Block(_) | Lexeme::Newline(_)) {
                    self.pending_newline = true;
                }
                Some(Token { lexeme, span })
            }
            Some(Err(_)) => None,
            None => None,
        }
    }
}

pub fn lex<'a>(source: &'a str) -> IndentAwareLexer<'a> {
    IndentAwareLexer::new(source)
}
