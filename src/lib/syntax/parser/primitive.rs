use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn identifier_node(&mut self) -> Result<IdentifierNode<'ast>> {
        let (start, end) = (self.current_span.start as u32, self.current_span.end as u32);
        let val = self.current_slice;
        self.expect(Token::Ident);
        Ok(self.node_at(start, end, val))
    }
}
