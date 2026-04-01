use std::ops::Range;
use logos::Logos;

#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(skip r"[\f]+")] // Ignore this regex pattern between tokens
pub enum Lexeme<'a> {
    #[regex(":(\\r?\\n)+")]
    Block(&'a str),

    #[regex("(\\r?\\n)+")]
    Newline(&'a str),

    #[regex("[ 　\\t]+")]
    Space(&'a str),

    #[regex("[0-9]+\\.[0-9]+|\\.[0-9]+|[0-9]+\\.")]
    Number(&'a str),

    #[regex("[0-9]+")]
    Integer(&'a str),

    #[regex("\\{[^\\}]*\\}")]
    Variable(&'a str),

    #[regex("\"[^\"]*\"")]
    String(&'a str),

    #[regex("<(?:\\w+)(?:\\s+[^>]*)?>")]
    XMLTagOpen(&'a str),

    #[regex("<\\/\\w+>")]
    XMLTagClose(&'a str),

    /**
    A token containing natural language text.

    Supported languages:

    * Afar (aa)
    * Afrikaans (af)
    * Aragonese (an)
    * Catalan (ca)
    * Corsican (co)
    * Czech (cs)
    * Welsh (cy)
    * Danish (da)
    * German (de)
    * English (en)
    * Esperanto (eo)
    * Spanish (es)
    * Finnish (fi)
    * Faroese (fo)
    * French (fr)
    * Western Frisian (fy)
    * Irish (ga)
    * Scottish Gaelic (gd)
    * Galician (gl)
    * Croatian (hr)
    * Hungarian (hu)
    * Icelandic (is)
    * Italian (it)
    * Japanese (ja)
    * Maltese (mt)
    * Dutch (nl)
    * Norwegian (no)
    * Polish (pl)
    * Portuguese (pt)
    * Romanian (ro)
    * Slovak (sk)
    * Slovenian (sl)
    * Swedish (sv)
    * Turkish (tr)
    * Ukrainian (uk)
    * Vietnamese (vi)
    */
    #[regex("([\\_]|(\\p{Script=Latin})+|(一-龯ぁ-んァ-ンー・))([\\_]|(\\p{Script=Latin})+|(一-龯ぁ-んァ-ンー・))*")]
    Text(&'a str),

    // #[regex("[\\:\\;\\.\\+\\-\\*\\/\\=<>]\\{\\}")]
    #[regex("[^\\w\\s]|[\\{\\}]")]
    Symbol(&'a str),

    Indent,

    Dedent,

    Error
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub lexeme: Lexeme<'a>,
    pub span: Range<usize>,
}

impl<'a> Token<'a> {
    pub fn new(lexeme: Lexeme<'a>, span: Range<usize>) -> Token<'a> {
        Token {
            lexeme,
            span
        }
    }

    pub fn lexeme_str(&self) -> &'a str {
        match self.lexeme {
            Lexeme::Block(l) => l,
            Lexeme::Newline(l) => l,
            Lexeme::Space(l) => l,
            Lexeme::Number(l) => l,
            Lexeme::Integer(l) => l,
            Lexeme::Variable(l) => l,
            Lexeme::String(l) => l,
            Lexeme::XMLTagOpen(l) => l,
            Lexeme::XMLTagClose(l) => l,
            Lexeme::Text(l) => l,
            Lexeme::Symbol(l) => l,
            Lexeme::Indent => "(indent)",
            Lexeme::Dedent => "(dedent)",
            Lexeme::Error => "(error)",
        }
    }
}
