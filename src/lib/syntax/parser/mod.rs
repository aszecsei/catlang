mod declaration;
mod expression;
mod primitive;
mod source;
mod statement;

use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::{Lexer, Token};
pub use logos::{lookup, Logos};
use toolshed::{list::GrowableList, Arena};

pub struct Parser<'ast> {
    arena: &'ast Arena,
    lexer: Lexer<'ast>,
    errors: Vec<Error>,
    body: SourceUnitList<'ast>,

    current_token: Token,
}

impl<'ast> Parser<'ast> {
    pub fn new(source: &str, arena: &'ast Arena) -> Self {
        let source = arena.alloc_nul_term_str(source);

        let mut lexer = Token::lexer(&source);
        let current_token = lexer.next().unwrap_or(Token::EndOfFile);

        let mut p = Parser {
            arena,
            lexer,
            errors: vec![],
            body: NodeList::empty(),
            current_token,
        };

        p.bump();

        p
    }

    fn bump(&mut self) {
        if self.current_token == Token::EndOfFile {
            return self.errors.push(Error::ExtendedBeyondEndOfFile);
        }
        self.current_token = self.lexer.next().unwrap_or(Token::EndOfFile);
    }

    #[inline]
    fn eat(&mut self, token: Token) -> bool {
        if self.current_token == token {
            self.bump();
            true
        } else {
            false
        }
    }

    #[inline]
    fn expect(&mut self, token: Token) {
        if self.current_token == token {
            self.bump();
        } else {
            self.errors.push(Error::ExpectedButGot {
                expected_token: token,
                token: self.current_token,
                raw: self.lexer.slice().into(),
                span: self.lexer.span(),
            })
        }
    }

    /// Expect next token to be edible or inedible token. If edible, then
    /// consume it; if inedible, return without consuming anything. Signal
    /// an error if next token is unexpected.
    #[inline]
    fn expect_one_of(&mut self, edible: &[Token], inedible: &[Token]) {
        if edible.contains(&self.current_token) {
            self.bump();
        } else if inedible.contains(&self.current_token) {
            //leave it in the input
        } else {
            let mut expected = edible.to_vec();
            expected.extend(inedible);
            self.errors.push(Error::ExpectedOneOfButGot {
                expected_tokens: expected,
                token: self.current_token,
                raw: self.lexer.slice().into(),
                span: self.lexer.span(),
            })
        }
    }

    #[inline]
    fn expect_eof(&mut self) {
        if self.current_token != Token::EndOfFile {
            self.errors.push(Error::ExpectedButGot {
                expected_token: Token::EndOfFile,
                token: self.current_token,
                raw: self.lexer.slice().into(),
                span: self.lexer.span(),
            })
        }
    }

    #[inline]
    fn expect_exact(&mut self, token: Token, expected: &str) {
        if self.current_token == token && self.lexer.slice() == expected {
            self.bump();
        } else {
            self.errors.push(Error::ExpectedButGot {
                expected_token: token,
                token: self.current_token,
                raw: self.lexer.slice().into(),
                span: self.lexer.span(),
            })
        }
    }

    #[inline]
    fn expect_end(&mut self, token: Token) -> u32 {
        let end = self.lexer.span().end as u32;
        self.expect(token);
        end
    }

    #[inline]
    fn str_node<R>(&mut self) -> R
    where
        R: From<Node<'ast, &'ast str>>,
    {
        let node = self.lexer.slice();
        self.node_at_token(node)
    }

    #[inline]
    fn expect_str_node(&mut self, token: Token) -> Node<'ast, &'ast str> {
        let val = self.lexer.slice();
        let (start, end) = self.loc();
        self.expect(token);
        self.node_at(start, end, val)
    }

    #[inline]
    fn allow_str_node(&mut self, token: Token) -> Option<Node<'ast, &'ast str>> {
        if self.current_token == token {
            self.str_node()
        } else {
            None
        }
    }

    #[inline]
    fn allow_flag_node(&mut self, token: Token) -> Option<FlagNode<'ast>> {
        if self.current_token == token {
            self.node_at_token(Flag)
        } else {
            None
        }
    }

    #[inline]
    fn loc(&mut self) -> (u32, u32) {
        let range = self.lexer.span();
        (range.start as u32, range.end as u32)
    }

    #[inline]
    fn start_then_advance(&mut self) -> u32 {
        let start = self.lexer.span().start as u32;
        self.bump();
        start
    }

    #[inline]
    fn end_then_advance(&mut self) -> u32 {
        let end = self.lexer.span().end as u32;
        self.bump();
        end
    }

    #[inline]
    fn alloc<T>(&mut self, val: NodeInner<T>) -> Node<'ast, T>
    where
        T: Copy,
    {
        Node::new(self.arena.alloc(val))
    }

    #[inline]
    fn node_at<T, I, R>(&mut self, start: u32, end: u32, item: I) -> R
    where
        T: 'ast + Copy,
        I: Into<T>,
        R: From<Node<'ast, T>>,
    {
        From::from(self.alloc(NodeInner::new(start, end, item.into())))
    }

    #[inline]
    fn node_at_token<T, I, R>(&mut self, item: I) -> R
    where
        T: 'ast + Copy,
        I: Into<T>,
        R: From<Node<'ast, T>>,
    {
        let (start, end) = self.loc();
        self.bump();
        self.node_at(start, end, item)
    }

    #[inline]
    fn node_from_slice<T, F, I, R>(&mut self, func: F) -> R
    where
        T: 'ast + Copy,
        F: FnOnce(&'ast str) -> I,
        I: Into<T>,
        R: From<Node<'ast, T>>,
    {
        let slice = self.lexer.slice();
        let (start, end) = self.loc();
        self.bump();
        self.node_at(start, end, func(slice))
    }

    #[inline]
    fn parse(&mut self) {
        let builder = GrowableList::new();
        builder.push(self.arena, self.source_unit());
        // while let Some(unit) = self.source_unit() {
        //     builder.push(self.arena, unit);
        // }
        self.body = builder.as_list();
        self.expect_eof();
    }

    #[inline]
    fn unique_flag<F>(&mut self, at: &mut Option<Node<'ast, F>>, flag: F)
    where
        F: Copy,
    {
        if at.is_some() {
            self.bump();
            return self.errors.push(Error::DuplicateFlagError {
                span: self.lexer.span(),
            });
        }

        *at = self.node_at_token(flag);
    }
}

pub fn parse<'src, 'ast>(source: &'src str) -> std::result::Result<Program<'ast>, Vec<Error>> {
    let arena = Arena::new();

    let (body, errors) = {
        let mut parser = Parser::new(source, &arena);
        parser.parse();
        (parser.body.into_unsafe(), parser.errors)
    };

    match errors.len() {
        0 => Ok(Program::new(body, arena)),
        _ => Err(errors),
    }
}
