use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn type_node(&mut self) -> Result<TypeExpressionNode<'ast>> {
        Err(Error::NotImplementedError)
    }
}
