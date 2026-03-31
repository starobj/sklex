use logos::Logos;

use crate::token::Token;

pub struct IndentAwareLexer<'a> {
    inner: logos::Lexer<'a, Token<'a>>,
    indent_stack: Vec<usize>,
    pending_newline: bool,
    indent_size: usize,
}

impl<'a> IndentAwareLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            inner: Token::lexer(source),
            indent_stack: vec![0],
            pending_newline: false,
            indent_size: 4,
        }
    }
}

impl<'a> Iterator for IndentAwareLexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pending_newline {
            self.pending_newline = false;
            // Peek at the next token without consuming it
            let mut temp_lexer = self.inner.clone();
            if let Some(Ok(Token::Space(lexeme))) = temp_lexer.next() {
                let space_len = lexeme.len();
                let current_indent = *self.indent_stack.last().unwrap();

                if space_len > current_indent {
                    let new_level = space_len / self.indent_size;
                    let old_level = current_indent / self.indent_size;
                    self.indent_stack.push(space_len);
                    for _ in old_level..new_level {
                        return Some(Token::Indent(lexeme));
                    }
                } else if space_len < current_indent {
                    let new_level = space_len / self.indent_size;
                    let old_level = current_indent / self.indent_size;
                    while *self.indent_stack.last().unwrap() > space_len {
                        self.indent_stack.pop();
                    }
                    for _ in new_level..old_level {
                        return Some(Token::Dedent(lexeme));
                    }
                }
                // Consume the Space token
                self.inner.next();
                return self.next();
            }
        }

        match self.inner.next() {
            Some(Ok(token)) => {
                if matches!(token, Token::Block(_) | Token::Newline(_)) {
                    self.pending_newline = true;
                }
                Some(token)
            }
            Some(Err(_)) => None,
            None => None,
        }
    }
}

pub fn lex<'a>(source: &'a str) -> IndentAwareLexer<'a> {
    IndentAwareLexer::new(source)
}
