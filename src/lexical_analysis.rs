use std::hash::{DefaultHasher, Hash, Hasher};
use logos::{Logos, Source};

use crate::token::*;

#[derive(Clone, Debug, PartialEq)]
enum LexerOp<'source> {
    /**
    An error was encountered.
     */
    Error,

    /**
    Lex a token.
     */
    Lex(Lexeme<'source>),

    /**
    Lex a newline token and any following indentation/dedentation.
     */
    Newline,

    /**
    Lex an indent.
     */
    Indent,

    /**
    Lex an dedent.
     */
    Dedent,

    /**
    End of File
     */
    Eof,
}

impl<'source> Hash for LexerOp<'source> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            LexerOp::Error => {
                0.hash(state);
            },
            LexerOp::Lex(token) => {
                1.hash(state);
                token.hash(state);
            }
            LexerOp::Newline => {
                2.hash(state);
            },
            LexerOp::Indent => {
                3.hash(state);
            },
            LexerOp::Dedent => {
                4.hash(state);
            },
            LexerOp::Eof => {
                5.hash(state);
            },
        }
    }
}

/**
A Skript source code lexical analyzer.
Used to tokenize Skript source code as a token iterator.
This implementation is a streaming lexical analyzer,
meaning that it returns an `Iterator` of `Token`s,
rather than a `Vec<Token>` or slice (`&[Token]`).

Lexers can be compared by token stream progress:

```rs
// Define some source code.
let source = "hello world";
// Create a couple of lexers: `a` and `b`.
let a = Lexer::new(source);
let b = Lexer::new(source);
// Read a token from `a`, but not `b`.
let token = a.next();
// Check which has processed more tokens:
if a > b {
    // `a` has lexed more tokens than `b`.
    // `a` is closer to the end than `b`.
    // Therefore, this block of code gets executed.
}
```
 */
#[derive(Clone, Debug)]
pub struct Lexer<'source> {
    op_count: usize,
    state_hasher: DefaultHasher,
    inner: logos::Lexer<'source, Lexeme<'source>>,
    indent_stack: Vec<usize>,
    pending_newline: bool,
    pending_indent_count: usize,
    pending_dedent_count: usize,
    pub indent_size: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            op_count: usize::MIN,
            state_hasher: DefaultHasher::new(),
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

    fn hash_op(&mut self, op: LexerOp) {
        op.hash(&mut self.state_hasher);

        self.op_count += 1;
    }
}

impl<'source> Hash for Lexer<'source> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.indent_stack.hash(state);
        // self.pending_newline.hash(state);
        // self.pending_indent_count.hash(state);
        // self.pending_dedent_count.hash(state);
        // self.indent_size.hash(state);
        self.state_hasher.clone().finish().hash(state);

        let lexemes = self.inner.clone().collect::<Vec<Result<Lexeme<'source>, ()>>>();

        for lexeme in lexemes {
            if lexeme.is_err() {
                // Write an empty lexeme.
                state.write_usize(0);

                // Continue the loop.
                continue;
            }

            lexeme.unwrap().hash(state);
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token<'source>;

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
                self.hash_op(LexerOp::Indent);
                return Some(
                    Token {
                        lexeme: Lexeme::Indent,
                        span: span.clone(),
                    }
                );
            }
            else {
                self.hash_op(LexerOp::Error);
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
                self.hash_op(LexerOp::Dedent);
                return Some(
                    Token {
                        lexeme: Lexeme::Dedent,
                        span,
                    }
                );
            }
            else {
                self.hash_op(LexerOp::Error);
                return Some(Token::new(Lexeme::Error, self.inner.span()));
            }
        }
        if self.pending_newline {
            self.pending_newline = false;
            if let Some(Ok(Lexeme::Space(slice))) = peeker.next() {
                let span = peeker.span();
                let space_length = slice.len();
                let current_indent_length = *self.indent_stack.last().unwrap();

                if space_length > current_indent_length {
                    let new_level = space_length / self.indent_size;
                    let old_level = current_indent_length / self.indent_size;
                    self.indent_stack.push(space_length);
                    self.pending_indent_count = new_level - old_level - 1;
                    // Consume the space.
                    self.inner.next();
                    self.hash_op(LexerOp::Indent);
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
                    self.hash_op(LexerOp::Dedent);
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
                self.hash_op(LexerOp::Dedent);
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
                self.hash_op(LexerOp::Lex(lexeme.clone()));
                let span = self.inner.span();
                match lexeme {
                    Lexeme::Block(_)
                    | Lexeme::Newline(_) => {
                        self.pending_newline = true;

                        self.hash_op(LexerOp::Newline);
                        Some(Token { lexeme, span })
                    }
                    Lexeme::String(slice) => {
                        let length = slice.chars().count();
                        Some(
                            Token::new(
                                Lexeme::String(
                                    // Remove the quotes that surround the string.
                                    slice.slice(1..(length - 1)).unwrap()
                                ),
                                span
                            )
                        )
                    },

                    Lexeme::Variable(slice) => {
                        let length = slice.chars().count();
                        Some(
                            Token::new(
                                Lexeme::Variable(
                                    // Remove the quotes that surround the string.
                                    slice.slice(1..(length - 1)).unwrap()
                                ),
                                span
                            )
                        )
                    }
                    _ => Some(Token { lexeme, span })
                }
            }
            Some(Err(_)) => {
                self.hash_op(LexerOp::Error);

                None
            },
            None => {
                self.hash_op(LexerOp::Eof);

                None
            },
        }
    }
}

impl<'source> PartialEq for Lexer<'source> {
    fn eq(&self, other: &Self) -> bool {
        // Hash self.
        let mut self_hasher = DefaultHasher::new();
        self.hash(&mut self_hasher);
        let self_hash = self_hasher.finish();

        // Hash other.
        let mut other_hasher = DefaultHasher::new();
        other.hash(&mut other_hasher);
        let other_hash = other_hasher.finish();

        // Compare hashes.
        self_hash == other_hash
    }
}

impl<'source> PartialOrd for Lexer<'source> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.op_count.partial_cmp(&other.op_count)
    }
}

pub fn lex<'source>(source: &'source str) -> Lexer<'source> {
    Lexer::new(source)
}
