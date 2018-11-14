use lexer::symbol::Symbol;
use std::fmt;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Token {
    Illegal(char),
    EOF,

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

            // Literals
            Token::Bool(b) => write!(f, "{}", b),
            Token::Ident(sym) => write!(f, "{}", sym),
            Token::Integer(val) => write!(f, "{}", val),
            Token::String(sym) => write!(f, "{}", sym),
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
        }
    }
}

impl Token {
    pub fn lookup_ident(ident: &str) -> Token {
        match ident {
            "let" => Token::Let,
            "const" => Token::Const,
            "new" => Token::New,
            "delete" => Token::Delete,
            "typeof" => Token::Typeof,
            "is" => Token::Is,
            "as" => Token::As,
            "in" => Token::In,
            "function" => Token::Function,
            "return" => Token::Return,
            "struct" => Token::Struct,
            "type" => Token::Type,
            "enum" => Token::Enum,
            "SOA" => Token::SOA,
            "owned" => Token::Owned,
            "import" => Token::Import,
            "export" => Token::Export,
            "from" => Token::From,
            "for" => Token::For,
            "while" => Token::While,
            "do" => Token::Do,
            "if" => Token::If,
            "else" => Token::Else,
            "break" => Token::Break,
            "continue" => Token::Continue,
            _ => Token::Ident(Symbol::intern(ident)),
        }
    }
}

#[test]
fn lookup_ident_test() {
    assert_eq!(Token::lookup_ident("function"), Token::Function);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Pos {
    NoPos,
    Pos(u32),
}

impl Default for Pos {
    fn default() -> Pos {
        Pos::NoPos
    }
}

impl Pos {
    fn is_valid(self) -> bool {
        self != Pos::NoPos
    }

    fn to_int(self) -> i32 {
        match self {
            Pos::NoPos => 0,
            Pos::Pos(x) => x as i32,
        }
    }
}

pub struct Position {
    filename: String,
    col: i32,
    row: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.filename {
            "" => write!(f, "{}:{}", self.row, self.col),
            fname => write!(f, "{}:{}:{}", fname, self.row, self.col),
        }
    }
}
