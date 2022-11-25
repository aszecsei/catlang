mod token;

pub use self::token::Token;
pub use logos::Logos;
pub type Lexer<'source> = logos::Lexer<'source, Token>;

#[cfg(test)]
mod test {
    use self::Token::*;
    use super::*;
    use logos::Logos;

    fn assert_lex<T>(source: &str, tokens: T)
    where
        T: AsRef<[(Token, &'static str)]>,
    {
        let mut lex = Token::lexer(source);

        for &(ref token, slice) in tokens.as_ref() {
            let lex_token = lex.next().expect("Unexpected end");
            assert!(
                lex_token == *token && lex.slice() == slice,
                "\n\n\n\tExpected {:?}({:?}), found {:?}({:?}) instead!\n\n\n",
                token,
                slice,
                lex_token,
                lex.slice()
            );
        }

        assert_eq!(lex.next(), None);
    }

    #[test]
    fn empty_lexer() {
        assert_lex("    ", []);
    }

    #[test]
    fn line_comment() {
        assert_lex(" // foo\nbar", [(Ident, "bar")]);
    }

    #[test]
    fn block_comment() {
        assert_lex(" /* foo */ bar", [(Ident, "bar")]);
        assert_lex(" /* foo **/ bar", [(Ident, "bar")]);
        assert_lex(" /* foo ***/ bar", [(Ident, "bar")]);
        assert_lex(" /* foo ****/ bar", [(Ident, "bar")]);
        assert_lex(" /* foo *****/ bar", [(Ident, "bar")]);

        assert_lex(" /* unterminated ", [(Error, "/* unterminated ")]);
    }

    #[test]
    fn identifiers() {
        assert_lex(
            "foo _foo $_foo _ $ $$ fooBar BarFoo foo10 $1",
            [
                (Ident, "foo"),
                (Ident, "_foo"),
                (Ident, "$_foo"),
                (Ident, "_"),
                (Ident, "$"),
                (Ident, "$$"),
                (Ident, "fooBar"),
                (Ident, "BarFoo"),
                (Ident, "foo10"),
                (Ident, "$1"),
            ],
        )
    }

    #[test]
    fn general_test() {
        assert_lex("; 21", [(Semicolon, ";"), (Integer(21), "21")])
    }
}
