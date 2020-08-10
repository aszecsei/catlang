use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    // pub fn statement_node(&mut self) -> Result<StatementNode<'ast>> {
    //     let start = self.loc().0;
    //     let statement = match self.current_token {
    //         Token::Import => self.import_statement()?,
    //         Token::LCurlyB => self.inner_block()?,
    //         Token::If => self.if_statement()?,
    //         Token::For => self.loop_statement()?,
    //         Token::While => self.loop_statement()?,
    //         Token::Do => self.loop_statement()?,
    //         Token::Loop => self.loop_statement()?,
    //         Token::Break => self.jump_statement()?,
    //         Token::Continue => self.jump_statement()?,
    //         Token::Return => self.jump_statement()?,
    //         Token::Delete => self.delete_statement()?,
    //         _ => self.expression_statement()?,
    //     };
    //     let end = self.last_span.end as u32;
    //
    //     Ok(self.node_at(start, end, statement))
    // }
    //
    // fn import_statement(&mut self) -> Result<Statement<'ast>> {
    //     // TODO: Implement imports
    //     Err(Error::NotImplementedError)
    // }
    //
    // fn inner_block(&mut self) -> Result<Statement<'ast>> {
    //     self.expect(Token::LCurlyB);
    //     let block_node = self.block_node();
    //     self.expect(Token::RCurlyB);
    //
    //     Ok(block_node.into())
    // }
    //
    // fn if_statement(&mut self) -> Result<Statement<'ast>> {
    //     self.expect(Token::If);
    //     self.expect(Token::LParen);
    //     let condition = self.expression_node()?;
    //     self.expect(Token::RParen);
    //     let true_block = self.statement_node()?;
    //     let else_block = if self.eat(Token::Else) {
    //         Some(self.statement_node()?)
    //     } else {
    //         None
    //     };
    //     Ok(IfStatement {
    //         condition,
    //         true_block,
    //         else_block,
    //     }
    //     .into())
    // }
    //
    // fn loop_statement(&mut self) -> Result<Statement<'ast>> {
    //     match self.current_token {
    //         Token::For => {
    //             // For loop
    //             self.expect(Token::For);
    //             self.expect(Token::LParen);
    //             if self.peek_token == Token::In {
    //                 // For-in loop
    //                 let identifier = self.identifier_node()?;
    //                 let range = self.expression_node()?;
    //                 let statement = self.statement_node()?;
    //                 Ok(Statement::Loop(
    //                     ForInLoop {
    //                         identifier,
    //                         range,
    //                         statement,
    //                     }
    //                     .into(),
    //                 ))
    //             } else {
    //                 // C-style For loop
    //                 let initial = if self.current_token != Token::Semicolon {
    //                     Some(self.expression_node()?)
    //                 } else {
    //                     None
    //                 };
    //                 self.expect(Token::Semicolon);
    //                 let condition = if self.current_token != Token::Semicolon {
    //                     Some(self.expression_node()?)
    //                 } else {
    //                     None
    //                 };
    //                 self.expect(Token::Semicolon);
    //                 let update = if self.current_token != Token::RParen {
    //                     Some(self.expression_node()?)
    //                 } else {
    //                     None
    //                 };
    //                 self.expect(Token::RParen);
    //                 let statement = self.statement_node()?;
    //                 Ok(Statement::Loop(
    //                     ForLoop {
    //                         initial,
    //                         condition,
    //                         update,
    //                         statement,
    //                     }
    //                     .into(),
    //                 ))
    //             }
    //         }
    //         Token::Do => {
    //             // do-while loop
    //             self.expect(Token::Do);
    //             let statement = self.statement_node()?;
    //             self.expect(Token::While);
    //             self.expect(Token::LParen);
    //             let condition = self.expression_node()?;
    //             self.expect(Token::RParen);
    //             Ok(Statement::Loop(
    //                 WhileLoop {
    //                     condition,
    //                     is_do_while: true,
    //                     statement,
    //                 }
    //                 .into(),
    //             ))
    //         }
    //         Token::While => {
    //             // while-loop
    //             self.expect(Token::While);
    //             self.expect(Token::LParen);
    //             let condition = self.expression_node()?;
    //             self.expect(Token::RParen);
    //             let statement = self.statement_node()?;
    //             Ok(Statement::Loop(
    //                 WhileLoop {
    //                     condition,
    //                     is_do_while: false,
    //                     statement,
    //                 }
    //                 .into(),
    //             ))
    //         }
    //         Token::Loop => {
    //             // infinite loop
    //             self.expect(Token::Loop);
    //             let statement = self.statement_node()?;
    //             Ok(Statement::Loop(InfiniteLoop { statement }.into()))
    //         }
    //         _ => Err(Error::NotImplementedError),
    //     }
    // }
    //
    // fn jump_statement(&mut self) -> Result<Statement<'ast>> {
    //     match self.current_token {
    //         Token::Break => Ok(JumpStatement::Break.into()),
    //         Token::Continue => Ok(JumpStatement::Continue.into()),
    //         Token::Return => {
    //             self.expect(Token::Return);
    //             let expression = self.expression_node()?;
    //             Ok(JumpStatement::Return(expression).into())
    //         }
    //         _ => Err(Error::NotImplementedError),
    //     }
    // }
    //
    // fn delete_statement(&mut self) -> Result<Statement<'ast>> {
    //     self.expect(Token::Delete);
    //     let expr = self.expression_node()?;
    //     Ok(DeleteStatement { deleted: expr }.into())
    // }
    //
    // fn expression_statement(&mut self) -> Result<Statement<'ast>> {
    //     let expr = self.expression_node()?;
    //     Ok(expr.into())
    // }
}
