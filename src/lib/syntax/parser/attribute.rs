use toolshed::list::GrowableList;

use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn attribute_node(&mut self) -> Result<AttributeNode<'ast>> {
        unimplemented!()
    }
}
