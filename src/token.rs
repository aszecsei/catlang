use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Token {
    Illegal,
    EOF,

    // Literals
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
            "if" => Token::If,
            "else" => Token::Else,
            "break" => Token::Break,
            "continue" => Token::Continue,
            _ => Token::Ident(ident.to_string()),
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
    fn is_valid(&self) -> bool {
        *self != Pos::NoPos
    }

    fn to_int(&self) -> i32 {
        return match *self {
            Pos::NoPos => 0,
            Pos::Pos(x) => x,
        } as i32;
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

pub struct File {
    base: i32,
    name: String,
    lines: Vec<i32>,
    size: i32,
}

impl File {
    pub fn new(name: String, base: i32, size: i32) -> File {
        return File {
            base: base,
            name: name,
            lines: vec![0; 16],
            size: size,
        };
    }

    pub fn add_line(&mut self, offset: i32) {
        if offset >= self.base - 1 && offset < self.base + self.size {
            self.lines.push(offset);
        }
    }

    pub fn get_pos(&self, offset: i32) -> Pos {
        if offset < 0 || offset >= self.size {
            panic!("Illegal file offset");
        }
        return Pos::Pos((self.base + offset) as u32);
    }

    pub fn get_position(&self, p: Pos) -> Position {
        let p_int = match p {
            Pos::NoPos => 0,
            Pos::Pos(v) => v,
        } as i32;
        let mut col = p_int + self.base + 1;
        let mut row: i32 = 1;

        for i in 0..self.lines.len() {
            let nl = self.lines[i];
            if p_int > self.get_pos(nl).to_int() {
                col = (p_int - self.get_pos(nl).to_int()) - self.base + 1;
                row = (i as i32) + 1;
            }
        }

        return Position {
            filename: self.name.to_string(),
            col: col,
            row: row,
        };
    }
}

pub struct FileSet {
    base: i32,
    files: Vec<File>,
}

impl FileSet {
    pub fn new() -> FileSet {
        return FileSet {
            base: 1,
            files: vec![],
        };
    }

    pub fn add(&mut self, name: String, src: String) -> &File {
        let f = File::new(name, self.base, src.len() as i32);
        self.files.push(f);
        self.base += src.len() as i32;

        match self.files.last() {
            None => panic!("No file found!"),
            Some(file) => return file,
        };
    }

    pub fn get_position(&self, p: Pos) -> Position {
        if !p.is_valid() {
            panic!("Invalid position");
        }
        let mut pos = Position {
            filename: String::from(""),
            col: 0,
            row: 0,
        };
        for f in self.files.iter() {
            if p.to_int() > self.base && p.to_int() < f.base + f.size {
                pos = f.get_position(p);
            }
        }
        return pos;
    }
}
