use id_arena::Id;
use std::fmt;

/// A symbol is an interned or gensymed string.
pub type Symbol = Id<String>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Token {
    Illegal(char),
    EOF,
    Whitespace,
    Comment,

    // Literals
    Bool(bool),
    Ident(Symbol),
    Integer(i32),
    String(Symbol),
    Char(char),

    // Operators
    LParen,
    RParen,
    LCurlyB,
    RCurlyB,
    LSquareB,
    RSquareB,
    Colon,
    DoubleColon,
    Semicolon,
    Comma,
    At,

    Add,
    AddAssign,
    Increment,
    Sub,
    SubAssign,
    Decrement,
    Mul,
    MulAssign,
    Quo,
    QuoAssign,
    Mod,
    ModAssign,

    Assign,

    And,
    AndAssign,
    BitAnd,
    BitAndAssign,
    Or,
    OrAssign,
    BitOr,
    BitOrAssign,
    Not,
    Xor,
    XorAssign,
    ShiftL,
    ShiftLAssign,
    ShiftR,
    ShiftRAssign,

    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,

    Arrow,
    Question,

    Dot,
    DotDot,

    // Keywords
    Let,
    Const,
    New,
    Delete,
    Typeof,
    Is,
    As,
    In,
    Function,
    Return,
    Struct,
    Type,
    Enum,
    SOA,
    Owned,
    Import,
    Export,
    From,
    For,
    While,
    Do,
    If,
    Else,
    Break,
    Continue,
    Null,
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal(' ')
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Illegal(ch) => write!(f, "Illegal character '{}'", ch),
            Token::EOF => write!(f, "EOF"),
            Token::Whitespace => write!(f, "Whitespace"),
            Token::Comment => write!(f, "Comment"),

            // Literals
            Token::Bool(b) => write!(f, "{}", b),
            Token::Ident(sym) => write!(f, "{:?}", sym),
            Token::Integer(val) => write!(f, "{}", val),
            Token::String(sym) => write!(f, "{:?}", sym),
            Token::Char(ch) => write!(f, "{}", ch),

            // Operators
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LCurlyB => write!(f, "{{"),
            Token::RCurlyB => write!(f, "}}"),
            Token::LSquareB => write!(f, "["),
            Token::RSquareB => write!(f, "]"),
            Token::Colon => write!(f, ":"),
            Token::DoubleColon => write!(f, "::"),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::At => write!(f, "@"),

            Token::Add => write!(f, "+"),
            Token::AddAssign => write!(f, "+="),
            Token::Increment => write!(f, "++"),
            Token::Sub => write!(f, "-"),
            Token::SubAssign => write!(f, "-="),
            Token::Decrement => write!(f, "--"),
            Token::Mul => write!(f, "*"),
            Token::MulAssign => write!(f, "*="),
            Token::Quo => write!(f, "/"),
            Token::QuoAssign => write!(f, "/="),
            Token::Mod => write!(f, "%"),
            Token::ModAssign => write!(f, "%="),

            Token::Assign => write!(f, "="),

            Token::And => write!(f, "&&"),
            Token::AndAssign => write!(f, "&&="),
            Token::BitAnd => write!(f, "&"),
            Token::BitAndAssign => write!(f, "&="),
            Token::Or => write!(f, "||"),
            Token::OrAssign => write!(f, "||="),
            Token::BitOr => write!(f, "|"),
            Token::BitOrAssign => write!(f, "|="),
            Token::Not => write!(f, "!"),
            Token::Xor => write!(f, "^"),
            Token::XorAssign => write!(f, "^="),
            Token::ShiftL => write!(f, "<<"),
            Token::ShiftLAssign => write!(f, "<<="),
            Token::ShiftR => write!(f, ">>"),
            Token::ShiftRAssign => write!(f, ">>="),

            Token::Equals => write!(f, "=="),
            Token::NotEquals => write!(f, "!="),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessThanEquals => write!(f, "<="),
            Token::GreaterThanEquals => write!(f, ">="),

            Token::Arrow => write!(f, "->"),
            Token::Question => write!(f, "?"),

            Token::Dot => write!(f, "."),
            Token::DotDot => write!(f, ".."),

            // Keywords
            Token::Let => write!(f, "let"),
            Token::Const => write!(f, "const"),
            Token::New => write!(f, "new"),
            Token::Delete => write!(f, "delete"),
            Token::Typeof => write!(f, "typeof"),
            Token::Is => write!(f, "is"),
            Token::As => write!(f, "as"),
            Token::In => write!(f, "in"),
            Token::Function => write!(f, "function"),
            Token::Return => write!(f, "return"),
            Token::Struct => write!(f, "struct"),
            Token::Type => write!(f, "type"),
            Token::Enum => write!(f, "enum"),
            Token::SOA => write!(f, "SOA"),
            Token::Owned => write!(f, "owned"),
            Token::Import => write!(f, "import"),
            Token::Export => write!(f, "export"),
            Token::From => write!(f, "from"),
            Token::For => write!(f, "for"),
            Token::While => write!(f, "while"),
            Token::Do => write!(f, "do"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::Null => write!(f, "null"),
        }
    }
}

impl Token {
    pub fn keyword_lookup(ident: &str) -> Option<Token> {
        match ident {
            "let" => Some(Token::Let),
            "const" => Some(Token::Const),
            "new" => Some(Token::New),
            "delete" => Some(Token::Delete),
            "typeof" => Some(Token::Typeof),
            "is" => Some(Token::Is),
            "as" => Some(Token::As),
            "in" => Some(Token::In),
            "function" => Some(Token::Function),
            "return" => Some(Token::Return),
            "struct" => Some(Token::Struct),
            "type" => Some(Token::Type),
            "enum" => Some(Token::Enum),
            "SOA" => Some(Token::SOA),
            "owned" => Some(Token::Owned),
            "import" => Some(Token::Import),
            "export" => Some(Token::Export),
            "from" => Some(Token::From),
            "for" => Some(Token::For),
            "while" => Some(Token::While),
            "do" => Some(Token::Do),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "break" => Some(Token::Break),
            "continue" => Some(Token::Continue),
            "null" => Some(Token::Null),
            _ => None,
        }
    }
}

#[test]
fn test_keyword_lookup() {
    assert_eq!(Token::keyword_lookup("function"), Some(Token::Function));
}
