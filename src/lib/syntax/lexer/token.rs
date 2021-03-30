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
    #[token("?.")]
    NullConditional,
    #[token("?[")]
    NullConditionalIndex,
    #[token("??")]
    NullCoalesce,
    #[token("??=")]
    NullCoalesceAssign,

    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,
    #[token("...")]
    DotDotDot,

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
    #[token("sizeof")]
    Sizeof,
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
    Soa,
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
    #[token("this")]
    This,
    #[token("volatile")]
    Volatile,

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
    #[token("c_short")]
    CShort,
    #[allow(clippy::upper_case_acronyms)]
    #[token("c_ushort")]
    CUShort,
    #[token("c_int")]
    CInt,
    #[allow(clippy::upper_case_acronyms)]
    #[token("c_uint")]
    CUInt,
    #[token("c_long")]
    CLong,
    #[allow(clippy::upper_case_acronyms)]
    #[token("c_ulong")]
    CULong,
    #[token("c_longlong")]
    CLongLong,
    #[allow(clippy::upper_case_acronyms)]
    #[token("c_ulonglong")]
    CULongLong,
    #[token("c_longdouble")]
    CLongDouble,
    #[token("bool")]
    BoolType,
    #[token("float")]
    Float,
    #[token("double")]
    Double,
    #[token("null")]
    Null,
    #[token("f32")]
    F32,
    #[token("f64")]
    F64,
    #[token("noreturn")]
    NoReturn,
    #[token("c_void")]
    CVoid,
}

impl Default for Token {
    fn default() -> Token {
        Token::Error
    }
}

fn skip_block_comment(lex: &mut Lexer<Token>) -> logos::Filter<()> {
    let remainder = lex.remainder();
    if let Some(idx) = remainder.find("*/") {
        lex.bump(idx + 2);
        return logos::Filter::Skip;
    }
    lex.bump(remainder.len());
    logos::Filter::Emit(())
}
