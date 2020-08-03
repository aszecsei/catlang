use logos::{Lexer, Logos};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Token {
    #[error]
    #[regex(r"\p{Whitespace}+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*", skip_block_comment)]
    Error,
    EndOfFile,

    // Literals
    #[regex("false|true", |lex| lex.slice() == "true")]
    Bool(bool),
    #[regex("[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Ident,
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Integer(i32),
    #[regex("\"(?:[^\"\\\\]|\\\\.)*\"")]
    LiteralString,
    #[regex("'([^'\\\\]|\\\\.)'")]
    LiteralChar,

    // Operators
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LCurlyB,
    #[token("}")]
    RCurlyB,
    #[token("#[")]
    LAttr,
    #[token("[")]
    LSquareB,
    #[token("]")]
    RSquareB,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("@")]
    At,

    #[token("+")]
    Add,
    #[token("+=")]
    AddAssign,
    #[token("++")]
    Increment,
    #[token("-")]
    Sub,
    #[token("-=")]
    SubAssign,
    #[token("--")]
    Decrement,
    #[token("*")]
    Mul,
    #[token("*=")]
    MulAssign,
    #[token("/")]
    Quo,
    #[token("/=")]
    QuoAssign,
    #[token("%")]
    Mod,
    #[token("%=")]
    ModAssign,

    #[token("=")]
    Assign,

    #[token("&&")]
    And,
    #[token("&&=")]
    AndAssign,
    #[token("&")]
    BitAnd,
    #[token("&=")]
    BitAndAssign,
    #[token("||")]
    Or,
    #[token("||=")]
    OrAssign,
    #[token("|")]
    BitOr,
    #[token("|=")]
    BitOrAssign,
    #[token("!")]
    Not,
    #[token("~")]
    BitNot,
    #[token("^")]
    Xor,
    #[token("^=")]
    XorAssign,
    #[token("<<")]
    ShiftL,
    #[token("<<=")]
    ShiftLAssign,
    #[token(">>")]
    ShiftR,
    #[token(">>=")]
    ShiftRAssign,

    #[token("==")]
    Equals,
    #[token("!=")]
    NotEquals,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessThanEquals,
    #[token(">=")]
    GreaterThanEquals,

    #[token("->")]
    Arrow,
    #[token("?")]
    Question,
    #[token("??")]
    NullCoalesce,
    #[token("??=")]
    NullCoalesceAssign,

    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,

    // Keywords
    #[token("any")]
    Any,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("new")]
    New,
    #[token("delete")]
    Delete,
    #[token("typeof")]
    Typeof,
    #[token("is")]
    Is,
    #[token("as")]
    As,
    #[token("in")]
    In,
    #[token("function")]
    Function,
    #[token("return")]
    Return,
    #[token("struct")]
    Struct,
    #[token("type")]
    Type,
    #[token("enum")]
    Enum,
    #[token("SOA")]
    SOA,
    #[token("owned")]
    Owned,
    #[token("import")]
    Import,
    #[token("export")]
    Export,
    #[token("from")]
    From,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("loop")]
    Loop,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("null")]
    Null,
    #[token("this")]
    This,

    // Type Primitives
    #[token("s8")]
    S8,
    #[token("u8")]
    U8,
    #[token("s16")]
    S16,
    #[token("u16")]
    U16,
    #[token("s32")]
    S32,
    #[token("u32")]
    U32,
    #[token("s64")]
    S64,
    #[token("u64")]
    U64,
    #[token("char")]
    Char,
    #[token("short")]
    Short,
    #[token("int")]
    Int,
    #[token("long")]
    Long,
    #[token("bool")]
    BoolType,
    #[token("float")]
    Float,
    #[token("double")]
    Double,
}

impl Default for Token {
    fn default() -> Token {
        Token::Error
    }
}

fn skip_block_comment<'source>(lex: &mut Lexer<'source, Token>) -> logos::Filter<()> {
    let remainder = lex.remainder();
    if let Some(idx) = remainder.find("*/") {
        lex.bump(idx + 2);
        return logos::Filter::Skip;
    }
    lex.bump(remainder.len());
    return logos::Filter::Emit(());
}
