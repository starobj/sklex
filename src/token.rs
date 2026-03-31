use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex(":(\\r?\\n)+")]
    Block,

    #[regex("(\\r?\\n)+")]
    Newline,

    #[regex("[ 　\\t]+")]
    Space,

    #[regex("[0-9]+\\.[0-9]+|\\.[0-9]+|[0-9]+\\.")]
    Number,

    #[regex("[0-9]+")]
    Integer,

    #[regex("\"[^\"]*\"")]
    String,

    #[regex("<(?:\\w+)(?:\\s+[^>]*)?>")]
    XMLTagOpen,

    #[regex("<\\/\\w+>")]
    XMLTagClose,

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
    Text,

    #[regex("[^\\w\\s]")]
    Symbol,

    Indent,

    Dedent,

    Error
}
