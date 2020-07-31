use toolshed::list::ListBuilder;

use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn statement_node(&mut self) -> Result<StatementNode<'ast>> {
        Err(Error::NotImplementedError)
    }
}
