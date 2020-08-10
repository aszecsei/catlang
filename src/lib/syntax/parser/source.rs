use toolshed::list::GrowableList;

use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn source_unit(&mut self) -> SourceUnitNode<'ast> {
        let start = self.current_span.start as u32;
        let module = Module {
            elements: GrowableList::new().as_list(),
        };
        let end = self.last_span.end as u32;
        self.node_at(start, end, module)
    }
}

fn is_declaration_starter(t: Token) -> bool {
    match t {
        Token::Export => true,
        Token::Const => true,
        Token::Type => true,
        Token::Let => true,
        Token::Function => true,
        Token::Struct => true,
        Token::Enum => true,
        _ => false,
    }
}
