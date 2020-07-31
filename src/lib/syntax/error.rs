use crate::syntax::lexer::Token;
use std::fmt::Debug;
use std::ops::Range;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("expected {:?} but got {:?} ({:?}) at {}:{}", expected_token, token, raw, span.start, span.end)]
    ExpectedButGot {
        expected_token: Token,
        token: Token,
        raw: Box<str>,
        span: Range<usize>,
    },

    #[error("expected one of {:?} but got {:?} ({:?}) at {}:{}", expected_tokens, token, raw, span.start, span.end)]
    ExpectedOneOfButGot {
        expected_tokens: Vec<Token>,
        token: Token,
        raw: Box<str>,
        span: Range<usize>,
    },

    #[error("parser requested token beyond end of file")]
    ExtendedBeyondEndOfFile,
    #[error("duplicate flag error at {}:{}", span.start, span.end)]
    DuplicateFlagError { span: Range<usize> },
    #[error("functionality not implemented")]
    NotImplementedError,
}

pub type Result<T> = std::result::Result<T, Error>;
