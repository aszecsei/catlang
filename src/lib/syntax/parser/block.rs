use toolshed::list::GrowableList;

use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::module::is_declaration_starter;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn block_node(&mut self) -> Result<BlockNode<'ast>> {
        let start = self.loc().0;
        self.expect(Token::LCurlyB);
        let elements = GrowableList::new();
        loop {
            match self.current_token {
                Token::RCurlyB => break,
                Token::Semicolon => self.bump(),
                _ => elements.push(self.arena, self.block_element()?),
            }
        }
        let end = self.loc().1;
        self.expect(Token::RCurlyB);
        Ok(self.node_at(
            start,
            end,
            Block {
                elements: elements.as_list(),
            },
        ))
    }

    fn block_element(&mut self) -> Result<BlockElementNode<'ast>> {
        if self.current_token == Token::Import {
            let import = self.import()?;
            Ok(self.node_at(import.start, import.end, import))
        } else if is_declaration_starter(self.current_token) {
            let declaration = self.declaration_node()?;
            Ok(self.node_at(declaration.start, declaration.end, declaration))
        } else {
            let statement = self.statement_node()?;
            Ok(self.node_at(statement.start, statement.end, statement))
        }
    }
}
