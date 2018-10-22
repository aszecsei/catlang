use token;
use token::Token;

use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Lexeme {
    token: Token,
    position: token::Pos,
}

pub struct Scanner<'a> {
    ch: Option<char>,
    offset: i32,
    src: Peekable<Chars<'a>>,
    file: &'a mut token::File,

    current_lexeme: Option<Lexeme>,
    next_lexeme: Option<Lexeme>,
}

impl<'a> Scanner<'a> {
    pub fn new(file: &'a mut token::File, src: &'a str) -> Scanner<'a> {
        let s = Scanner {
            ch: None,
            offset: 0,
            src: src.chars().peekable(),
            file: file,

            current_lexeme: Default::default(),
            next_lexeme: Default::default(),
        };
        s.file.add_line(s.offset);
        return s;
    }

    pub fn advance(mut self) {
        self.current_lexeme = self.next_lexeme;
        self.next_lexeme = None;
        self.scan();
    }

    fn scan(&mut self) {
        self.skip_whitespace();

        // Skip comments - note we need to re-call scan in case
        // there are multiple consecutive comments/whitespace combinations
        if self.ch == Some('/') && self.peek_char_eq('/') {
            self.skip_comment(false);
            self.scan();
            return;
        }
        if self.ch == Some('/') && self.peek_char_eq('*') {
            self.skip_comment(true);
            self.scan();
            return;
        }

        let pos = self.file.get_pos(self.offset);

        let t = match self.ch {
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LCurlyB,
            Some('}') => Token::RCurlyB,
            Some('[') => Token::LSquareB,
            Some(']') => Token::RSquareB,
            Some(':') => {
                if self.peek_char_eq(':') {
                    self.read_char();
                    Token::DoubleColon
                } else {
                    Token::Colon
                }
            }
            Some(';') => Token::Semicolon,
            Some(',') => Token::Comma,
            Some('@') => Token::At,
            Some('+') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::AddAssign
                } else if self.peek_char_eq('+') {
                    self.read_char();
                    Token::Increment
                } else {
                    Token::Add
                }
            }
            Some('-') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::SubAssign
                } else if self.peek_char_eq('-') {
                    self.read_char();
                    Token::Decrement
                } else if self.peek_char_eq('>') {
                    self.read_char();
                    Token::Arrow
                } else {
                    Token::Sub
                }
            }
            Some('*') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::MulAssign
                } else {
                    Token::Mul
                }
            }
            Some('/') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::QuoAssign
                } else {
                    Token::Quo
                }
            }
            Some('%') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::ModAssign
                } else {
                    Token::Mod
                }
            }
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equals
                } else {
                    Token::Assign
                }
            }
            Some('&') => {
                if self.peek_char_eq('&') {
                    self.read_char();
                    if self.peek_char_eq('=') {
                        self.read_char();
                        Token::AndAssign
                    } else {
                        Token::And
                    }
                } else {
                    if self.peek_char_eq('=') {
                        self.read_char();
                        Token::BitAndAssign
                    } else {
                        Token::BitAnd
                    }
                }
            }
            Some('|') => {
                if self.peek_char_eq('|') {
                    self.read_char();
                    if self.peek_char_eq('=') {
                        self.read_char();
                        Token::OrAssign
                    } else {
                        Token::Or
                    }
                } else {
                    if self.peek_char_eq('=') {
                        self.read_char();
                        Token::BitOrAssign
                    } else {
                        Token::BitOr
                    }
                }
            }
            Some('^') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::XorAssign
                } else {
                    Token::Xor
                }
            }
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEquals
                } else {
                    Token::Not
                }
            }
            Some('<') => {
                if self.peek_char_eq('<') {
                    self.read_char();
                    if self.peek_char_eq('=') {
                        self.read_char();
                        Token::ShiftLAssign
                    } else {
                        Token::ShiftL
                    }
                } else if self.peek_char_eq('=') {
                    self.read_char();
                    Token::LessThanEquals
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                if self.peek_char_eq('>') {
                    self.read_char();
                    if self.peek_char_eq('=') {
                        self.read_char();
                        Token::ShiftRAssign
                    } else {
                        Token::ShiftR
                    }
                } else if self.peek_char_eq('=') {
                    self.read_char();
                    Token::GreaterThanEquals
                } else {
                    Token::GreaterThan
                }
            }
            Some('?') => Token::Optional,
            Some('.') => {
                if self.peek_char_eq('.') {
                    self.read_char();
                    Token::DotDot
                } else {
                    Token::Dot
                }
            }
            Some('\'') => Token::Char('\n'), // TODO: Scan char literal
            Some('"') => Token::String(String::from("")), // TODO: Scan string literal

            Some(ch) => {
                if is_letter(ch) {
                    let literal = self.scan_identifier();
                    Token::lookup_ident(&literal)
                } else if ch.is_numeric() {
                    self.scan_number()
                } else {
                    Token::Illegal // TODO: we may need to display a nice error message later
                }
            }
            None => Token::EOF,
        };

        self.next_lexeme = Some(Lexeme {
            token: t,
            position: pos,
        });
    }

    fn scan_number(&mut self) -> Token {
        // TODO: Handle floats
        // TODO: Handle number bases other than 10
        // TODO
        Token::Integer(0)
    }

    fn scan_identifier(&mut self) -> String {
        let ident = String::new();
        // TODO
        return ident;
    }

    fn read_char(&mut self) -> Option<char> {
        self.offset += 1;
        self.ch = self.src.next();
        if self.ch == Some('\n') {
            self.file.add_line(self.offset);
        }
        return self.ch;
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.src.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.read_char() {
            if !c.is_whitespace() {
                break;
            }
        }
    }

    fn skip_comment(&mut self, is_block: bool) {
        self.read_char(); // pass through /
        self.read_char(); // pass through * or /
        while let Some(c) = self.read_char() {
            if is_block && c == '*' && self.peek_char_eq('/') {
                self.read_char(); // pass through *
                self.read_char(); // pass through /
                break;
            }
            if !is_block && c == '\n' {
                self.read_char(); // pass through \n
                break;
            }
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
