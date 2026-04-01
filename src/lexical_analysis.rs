use logos::{Lexer, Logos};

use crate::{Lexeme, Token};

pub struct IndentAwareLexer<'a> {
    inner: logos::Lexer<'a, Lexeme<'a>>,
    indent_stack: Vec<usize>,
    pending_newline: bool,
    pending_indent_count: usize,
    pending_dedent_count: usize,
    indent_size: usize,
    indent_lexeme: String,
}

impl<'a> IndentAwareLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Self {
            inner: Lexeme::lexer(source),
            indent_stack: vec![0],
            pending_newline: false,
            pending_indent_count: 0,
            pending_dedent_count: 0,
            indent_size: 0,
            indent_lexeme: String::new(),
        };

        lexer.set_indent_size(4);

        lexer
    }

    pub fn set_indent_size(&mut self, indent_size: usize) {
        self.indent_size = indent_size;
        self.indent_lexeme = String::from(" ".repeat(indent_size).as_str());
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
                println!("Indent({} -> {})", old_level, new_level);
                self.indent_stack.push(space_length);
                self.pending_indent_count = new_level - old_level - 1;
                return Some(
                    Token {
                        lexeme: Lexeme::Indent(self.indent_lexeme.as_str()),
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
                println!("Dedent({} -> {})", old_level, new_level);
                self.indent_stack.pop();
                self.pending_indent_count = new_level - old_level - 1;
                return Some(
                    Token {
                        lexeme: Lexeme::Dedent(""),
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
            println!("pending_newline == true");
            if let Some(Ok(Lexeme::Space(lexeme))) = peeker.next() {
                println!("Indentation encountered: {:?}", lexeme);
                let span = peeker.span();
                let space_length = lexeme.len();
                let current_indent_length = *self.indent_stack.last().unwrap();

                if space_length > current_indent_length {
                    let new_level = space_length / self.indent_size;
                    let old_level = current_indent_length / self.indent_size;
                    println!("Indent({} -> {})", old_level, new_level);
                    self.indent_stack.push(space_length);
                    self.pending_indent_count = new_level - old_level - 1;
                    return Some(
                        Token {
                            lexeme: Lexeme::Indent(lexeme),
                            span: span.clone(),
                        }
                    );
                } else if space_length < current_indent_length {
                    let new_level = space_length / self.indent_size;
                    let old_level = current_indent_length / self.indent_size;
                    println!("Dedent({} -> {})", old_level, new_level);
                    // while *self.indent_stack.last().unwrap() > space_len {
                    //     self.indent_stack.pop();
                    // }
                    // for _ in new_level..old_level {
                    //     return Some(Token {
                    //         lexeme: Lexeme::Dedent(lexeme),
                    //         span: span.clone(),
                    //     });
                    // }
                    self.pending_dedent_count = old_level - new_level - 1;
                    self.indent_stack.pop();
                    return Some(
                        Token {
                            lexeme: Lexeme::Indent(lexeme),
                            span: span.clone(),
                        }
                    );
                }
                // Consume the Space token
                self.inner.next();
                return self.next();
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
