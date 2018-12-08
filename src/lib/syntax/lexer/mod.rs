use log::{debug, error};
use std::iter::Peekable;
use std::ops::Deref;
use std::rc::Rc;
use std::str::Chars;

use crate::syntax::context::Context;
use crate::syntax::source_map::DUMMY_SPAN;
use crate::syntax::source_map::{Pos, SourceFile, Span};
use crate::syntax::token::Token;

#[derive(Clone, Debug)]
pub struct TokenAndSpan {
    pub tok: Token,
    pub sp: Span,
}

impl Deref for TokenAndSpan {
    type Target = Token;

    fn deref(&self) -> &Token {
        &self.tok
    }
}

pub struct StringReader<'a> {
    ctx: &'a mut Context,
    ch: Option<char>,
    next_pos: Pos,
    pos: Pos,
    source_file: Rc<SourceFile>,
    src_raw: &'a str,
    src: Peekable<Chars<'a>>,
    token: Token,
    span: Span,
    peek_tok: Token,
    peek_span: Span,
}

// TODO: Look into turning this into an iterator
impl<'a> StringReader<'a> {
    pub fn token(&self) -> TokenAndSpan {
        TokenAndSpan {
            tok: self.token,
            sp: self.span,
        }
    }

    pub fn peek(&self) -> TokenAndSpan {
        TokenAndSpan {
            tok: self.peek_tok,
            sp: self.peek_span,
        }
    }

    pub fn new(ctx: &'a mut Context, source_file: Rc<SourceFile>, src: &'a str) -> Self {
        let ps = src.chars().peekable();
        let mut sr = StringReader {
            ctx,
            next_pos: source_file.start_pos(),
            pos: source_file.start_pos(),
            ch: Some('\n'),
            source_file: source_file.clone(),
            peek_tok: Token::EOF,
            peek_span: DUMMY_SPAN,
            src_raw: src,
            src: ps,
            token: Token::EOF,
            span: DUMMY_SPAN,
        };
        if sr.advance_token().is_err() {
            error!("Uh oh!"); // TODO: Improve error reporting
        }
        sr
    }

    pub fn next(&mut self) -> TokenAndSpan {
        self.token = self.peek_tok;
        self.span = self.peek_span;
        self.advance_token().expect("Failed to advance");
        self.token()
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.ch.is_none()
    }

    fn bump(&mut self) {
        if self.next_pos < self.source_file.end_pos() {
            let next_ch = self.src.next().unwrap_or('\0');
            let next_ch_len = next_ch.len_utf8();

            self.ch = Some(next_ch);
            self.pos = self.next_pos;
            self.next_pos = self.next_pos + Pos::from_usize(next_ch_len);
        } else {
            self.ch = None;
            self.pos = self.next_pos;
        }
    }

    fn nextch(&mut self) -> Option<char> {
        let res = self.src.peek();
        match res {
            Some(&peek_ch) => Some(peek_ch),
            None => None,
        }
    }

    #[inline]
    fn nextch_is(&mut self, c: char) -> bool {
        self.nextch() == Some(c)
    }

    #[inline]
    fn ch_is(&self, ch: char) -> bool {
        self.ch == Some(ch)
    }

    fn advance_token(&mut self) -> Result<(), ()> {
        match self.scan_whitespace_or_comment() {
            Some(comment) => {
                self.peek_span = comment.sp;
                self.peek_tok = comment.tok;
            }
            None => {
                if self.is_eof() {
                    self.peek_tok = Token::EOF;
                    self.peek_span = Span {
                        low: self.source_file.end_pos(),
                        high: self.source_file.end_pos(),
                    };
                } else {
                    let start_pos = self.pos;
                    self.peek_tok = self.next_token_inner()?;
                    self.peek_span = Span {
                        low: start_pos,
                        high: self.pos,
                    };
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn src_index(&self, pos: Pos) -> usize {
        (pos - self.source_file.start_pos()) as usize
    }

    fn scan_comment(&mut self) -> Option<TokenAndSpan> {
        if let Some(c) = self.ch {
            if c.is_whitespace() {
                error!("Called consume_any_line_comment, but there was whitespace");
            }
        }

        if self.ch_is('/') {
            match self.nextch() {
                Some('/') => {
                    let start_pos = self.pos;
                    self.bump();
                    self.bump(); // Skip both slashes
                    while !self.is_eof() {
                        match self.ch.unwrap() {
                            '\n' => break,
                            '\r' => {
                                if self.nextch_is('\n') {
                                    // CRLF
                                    break;
                                }
                            }
                            _ => (),
                        }
                        self.bump();
                    }

                    Some(TokenAndSpan {
                        tok: Token::Comment,
                        sp: Span {
                            low: start_pos,
                            high: self.pos,
                        },
                    })
                }
                Some('*') => {
                    let start_pos = self.pos;
                    self.bump();
                    self.bump(); // Skip the /*
                    while !self.is_eof() {
                        match self.ch.unwrap() {
                            // TODO: Look into level-based block comments
                            '*' => {
                                if self.nextch_is('/') {
                                    self.bump();
                                    self.bump();
                                    break;
                                }
                            }
                            _ => (),
                        }
                        self.bump();
                    }

                    Some(TokenAndSpan {
                        tok: Token::Comment,
                        sp: Span {
                            low: start_pos,
                            high: self.pos,
                        },
                    })
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn scan_whitespace_or_comment(&mut self) -> Option<TokenAndSpan> {
        match self.ch.unwrap_or('\0') {
            '/' => {
                let c = self.scan_comment();
                debug!("Scanning a comment {:?}", c);
                c
            }
            c if c.is_whitespace() => {
                let start_pos = self.pos;
                while self.ch.map_or(false, |c| c.is_whitespace()) {
                    self.bump();
                }
                let c = Some(TokenAndSpan {
                    tok: Token::Whitespace,
                    sp: Span {
                        low: start_pos,
                        high: self.pos,
                    },
                });
                debug!("Scanning whitespace: {:?}", c);
                c
            }
            _ => None,
        }
    }

    fn expect(&mut self, c: char) -> Result<(), ()> {
        if self.ch != Some(c) {
            self.bump();
            Err(())
        } else {
            self.bump();
            Ok(())
        }
    }

    /// Return the next token from the string, advance the input past that token, and update the interner
    fn next_token_inner(&mut self) -> Result<Token, ()> {
        let c = self.ch;

        // Check for identifiers or keywords
        if ident_start(c) {
            let start = self.pos;
            self.bump();

            while ident_continue(self.ch) {
                self.bump();
            }

            let string = &self.src_raw[self.src_index(start)..self.src_index(self.pos)];
            let t = Token::keyword_lookup(string);
            let result = match t {
                Some(t) => t,
                None => {
                    debug!("Scanned identifier {}", string);
                    let ident = self.ctx.intern(string);
                    Token::Ident(ident)
                }
            };

            return Ok(result);
        }

        // Check for numerals
        if is_dec_digit(c) {
            return self.scan_number();
        }

        // Scan punctuation
        self.scan_punctuation(c)
    }

    fn scan_punctuation(&mut self, c: Option<char>) -> Result<Token, ()> {
        match c.expect("next_token_inner called at EOF") {
            // One-byte tokens
            ';' => {
                self.bump();
                Ok(Token::Semicolon)
            }
            '(' => {
                self.bump();
                Ok(Token::LParen)
            }
            ')' => {
                self.bump();
                Ok(Token::RParen)
            }
            '{' => {
                self.bump();
                Ok(Token::LCurlyB)
            }
            '}' => {
                self.bump();
                Ok(Token::RCurlyB)
            }
            '[' => {
                self.bump();
                Ok(Token::LSquareB)
            }
            ']' => {
                self.bump();
                Ok(Token::RSquareB)
            }
            ',' => {
                self.bump();
                Ok(Token::Comma)
            }
            '@' => {
                self.bump();
                Ok(Token::At)
            }
            '?' => {
                self.bump();
                Ok(Token::Question)
            }
            // Multi-char options
            ':' => {
                if self.nextch_is(':') {
                    self.bump();
                    self.bump();
                    Ok(Token::DoubleColon)
                } else {
                    self.bump();
                    Ok(Token::Colon)
                }
            }
            '+' => {
                if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::AddAssign)
                } else if self.nextch_is('+') {
                    self.bump();
                    self.bump();
                    Ok(Token::Increment)
                } else {
                    self.bump();
                    Ok(Token::Add)
                }
            }
            '-' => {
                if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::SubAssign)
                } else if self.nextch_is('-') {
                    self.bump();
                    self.bump();
                    Ok(Token::Decrement)
                } else if self.nextch_is('>') {
                    self.bump();
                    self.bump();
                    Ok(Token::Arrow)
                } else {
                    self.bump();
                    Ok(Token::Sub)
                }
            }
            '*' => {
                if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::MulAssign)
                } else {
                    self.bump();
                    Ok(Token::Mul)
                }
            }
            '/' => {
                if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::QuoAssign)
                } else {
                    self.bump();
                    Ok(Token::Quo)
                }
            }
            '%' => {
                if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::ModAssign)
                } else {
                    self.bump();
                    Ok(Token::Mod)
                }
            }
            '=' => {
                if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::Equals)
                } else {
                    self.bump();
                    Ok(Token::Assign)
                }
            }
            '&' => {
                if self.nextch_is('&') {
                    self.bump();
                    if self.nextch_is('=') {
                        self.bump();
                        self.bump();
                        Ok(Token::AndAssign)
                    } else {
                        self.bump();
                        Ok(Token::And)
                    }
                } else if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::BitAndAssign)
                } else {
                    self.bump();
                    Ok(Token::BitAnd)
                }
            }
            '|' => {
                if self.nextch_is('|') {
                    self.bump();
                    if self.nextch_is('=') {
                        self.bump();
                        self.bump();
                        Ok(Token::OrAssign)
                    } else {
                        self.bump();
                        Ok(Token::Or)
                    }
                } else if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::BitOrAssign)
                } else {
                    self.bump();
                    Ok(Token::BitOr)
                }
            }
            '^' => {
                if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::XorAssign)
                } else {
                    self.bump();
                    Ok(Token::Xor)
                }
            }
            '!' => {
                if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::NotEquals)
                } else {
                    self.bump();
                    Ok(Token::Not)
                }
            }
            '.' => {
                if self.nextch_is('.') {
                    self.bump();
                    self.bump();
                    Ok(Token::DotDot)
                } else {
                    self.bump();
                    Ok(Token::Dot)
                }
            }
            '<' => {
                if self.nextch_is('<') {
                    self.bump();
                    if self.nextch_is('=') {
                        self.bump();
                        self.bump();
                        Ok(Token::ShiftLAssign)
                    } else {
                        self.bump();
                        Ok(Token::ShiftL)
                    }
                } else if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::LessThanEquals)
                } else {
                    self.bump();
                    Ok(Token::LessThan)
                }
            }
            '>' => {
                if self.nextch_is('>') {
                    self.bump();
                    if self.nextch_is('=') {
                        self.bump();
                        self.bump();
                        Ok(Token::ShiftRAssign)
                    } else {
                        self.bump();
                        Ok(Token::ShiftR)
                    }
                } else if self.nextch_is('=') {
                    self.bump();
                    self.bump();
                    Ok(Token::GreaterThanEquals)
                } else {
                    self.bump();
                    Ok(Token::GreaterThan)
                }
            }
            '\'' => self.scan_char_literal(),
            '"' => self.scan_string_literal(),
            _ => Err(()),
        }
    }

    fn scan_char_literal(&mut self) -> Result<Token, ()> {
        self.bump(); // Skip over the '
        let t = Token::Char(self.ch.unwrap());
        self.expect('\'')?; // Skip over the other '
        Ok(t)
    }

    fn scan_string_literal(&mut self) -> Result<Token, ()> {
        self.bump();
        let start = self.pos;

        while self.ch != Some('"') {
            self.bump();
        }

        let string = &self.src_raw[self.src_index(start)..self.src_index(self.pos)];
        let interned_string = self.ctx.intern(string);
        self.expect('"')?;
        debug!("Reading string \"{}\"", string);

        Ok(Token::String(interned_string))
    }

    fn scan_number(&mut self) -> Result<Token, ()> {
        let mut value: i32 = 0;
        while is_dec_digit(self.ch) {
            let c = self.ch.unwrap();
            let v = c.to_digit(10);
            if let Some(v_d) = v {
                value *= 10;
                value += v_d as i32;
            } else {
                break;
            }
            self.bump();
        }
        Ok(Token::Integer(value))
    }
}

#[inline]
fn in_range(c: Option<char>, low: char, high: char) -> bool {
    c.map_or(false, |c| low <= c && c <= high)
}

#[inline]
fn is_dec_digit(c: Option<char>) -> bool {
    in_range(c, '0', '9')
}

fn ident_start(c: Option<char>) -> bool {
    let c = match c {
        Some(c) => c,
        None => return false,
    };

    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' // TODO: Handle unicode
}

fn ident_continue(c: Option<char>) -> bool {
    let c = match c {
        Some(c) => c,
        None => return false,
    };

    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_' // TODO: Handle unicode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let src = "; 21";
        let mut context = Context::new();
        let sf = context
            .get_source_map()
            .add_file(String::from("test.cat"), String::from(src));

        let mut sr = StringReader::new(&mut context, sf, src);
        sr.next();
        assert_eq!(sr.next().tok, Token::Semicolon);
        assert_eq!(sr.next().tok, Token::Whitespace);
        assert_eq!(sr.next().tok, Token::Integer(21));
    }
}
