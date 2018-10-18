use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Token {
    Illegal,
    EOF,

    // Literals
    _litStart,
    Bool(bool),
    Ident(String),
    Integer(i32),
    String(String),
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
    Comm,
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
    GreatherThan,
    LessThanEquals,
    GreaterThanEquals,

    Arrow,
    Optional,

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
    If,
    Else,
    Break,
    Continue,
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

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
        "if" => Token::If,
        "else" => Token::Else,
        "break" => Token::Break,
        "continue" => Token::Continue,
        _ => Token::Ident(ident.to_string()),
    }
}

#[test]
fn lookup_ident_test() {
    assert_eq!(lookup_ident("function"), Token::Function);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Pos {
    NoPos,
    Pos(u32),
}

impl Pos {
    fn is_valid(self) -> bool {
        self != Pos::NoPos
    }
}

pub struct Position {
    filename: String,
    col: i32,
    row: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &*self.filename {
            "" => write!(f, "{}:{}", self.row, self.col),
            fname => write!(f, "{}:{}:{}", fname, self.row, self.col),
        };
    }
}
