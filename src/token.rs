use logos::Logos;

#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(skip r"[\f]+")] // Ignore this regex pattern between tokens
pub enum Token<'a> {
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
    #[regex("(\\p{Script=Latin}+)|(一-龯ぁ-んァ-ンー・)")]
    Text(&'a str),

    #[regex("[^\\w\\s]")]
    Symbol(&'a str),

    Indent(&'a str),

    Dedent(&'a str),

    Error
}
