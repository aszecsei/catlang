use toolshed::list::GrowableList;

use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn source_unit(&mut self) -> SourceUnitNode<'ast> {
        let start = self.loc().0;
        let block = self.block_node();
        let end = self.last_span.end as u32;
        self.node_at(start, end, SourceUnit { block })
    }

    pub fn block_node(&mut self) -> BlockNode<'ast> {
        let start = self.loc().0;
        let elements = GrowableList::new();
        loop {
            match self.current_token {
                Token::EndOfFile => break,
                Token::RCurlyB => break,
                Token::Semicolon => {
                    self.bump();
                }
                _ => {
                    let res = self.block_element_node();
                    if let Ok(element_node) = res {
                        elements.push(self.arena, element_node);
                    }
                }
            }
        }
        let end = self.last_span.end as u32;
        self.node_at(
            start,
            end,
            Block {
                elements: elements.as_list(),
            },
        )
    }

    fn block_element_node(&mut self) -> Result<Node<'ast, BlockElement<'ast>>> {
        let start = self.loc().0;
        let block_element = if is_declaration_starter(self.current_token) {
            BlockElement::Declaration(self.declaration_node()?)
        } else {
            BlockElement::Statement(self.statement_node()?)
        };
        let end = self.last_span.end as u32;
        Ok(self.node_at(start, end, block_element))
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
