use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn statement_node(&mut self) -> Result<StatementNode<'ast>> {
        match self.current_token {
            Token::LCurlyB => self.inner_block(),
            Token::If => self.if_statement(),
            Token::While | Token::Do | Token::For | Token::Loop => self.loop_statement(),
            Token::Break | Token::Continue | Token::Return => self.jump_statement(),
            Token::Delete => self.delete_statement(),
            _ => self.expression_statement(),
        }
    }

    fn inner_block(&mut self) -> Result<StatementNode<'ast>> {
        let block = self.block_node()?;
        Ok(self.node_at(block.start, block.end, block))
    }

    fn if_statement(&mut self) -> Result<StatementNode<'ast>> {
        let start = self.start_then_advance();
        self.expect(Token::LParen);
        let condition = self.expression_node()?;
        self.expect(Token::RParen);
        let true_block = self.statement_node()?;
        let mut end = true_block.end;
        let else_block = if self.eat(Token::Else) {
            let res = self.statement_node()?;
            end = res.end;
            Some(res)
        } else {
            None
        };
        Ok(self.node_at(
            start,
            end,
            IfStatement {
                condition,
                true_block,
                else_block,
            },
        ))
    }

    fn loop_statement(&mut self) -> Result<StatementNode<'ast>> {
        match self.current_token {
            Token::For => {
                // for loop
                let start = self.start_then_advance();
                self.expect(Token::LParen);
                let identifier = self.identifier_node()?;
                self.expect(Token::In);
                let range = self.expression_node()?;
                self.expect(Token::RParen);
                let statement = self.statement_node()?;
                Ok(self.node_at(
                    start,
                    statement.end,
                    Statement::Loop(
                        ForLoop {
                            identifier,
                            range,
                            statement,
                        }
                        .into(),
                    ),
                ))
            }
            Token::Do => {
                // do-while loop
                let start = self.start_then_advance();
                let statement = self.statement_node()?;
                self.expect(Token::While);
                self.expect(Token::LParen);
                let condition = self.expression_node()?;
                let end = self.expect_end(Token::RParen);
                Ok(self.node_at(
                    start,
                    end,
                    Statement::Loop(
                        WhileLoop {
                            condition,
                            is_do_while: true,
                            statement,
                        }
                        .into(),
                    ),
                ))
            }
            Token::While => {
                // while loop
                let start = self.start_then_advance();
                self.expect(Token::LParen);
                let condition = self.expression_node()?;
                self.expect(Token::RParen);
                let statement = self.statement_node()?;
                Ok(self.node_at(
                    start,
                    statement.end,
                    Statement::Loop(
                        WhileLoop {
                            condition,
                            is_do_while: false,
                            statement,
                        }
                        .into(),
                    ),
                ))
            }
            Token::Loop => {
                // infinite loop
                let start = self.start_then_advance();
                let statement = self.statement_node()?;
                Ok(self.node_at(
                    start,
                    statement.end,
                    Statement::Loop(InfiniteLoop { statement }.into()),
                ))
            }
            t => Err(Error::ExpectedOneOfButGot {
                expected_tokens: vec![Token::For, Token::While, Token::Do, Token::Loop],
                token: t,
                raw: self.current_slice.into(),
                span: self.current_span.clone(),
            }),
        }
    }

    fn jump_statement(&mut self) -> Result<StatementNode<'ast>> {
        let res = match self.current_token {
            Token::Break => Ok(self.node_at_token(JumpStatement::Break)),
            Token::Continue => Ok(self.node_at_token(JumpStatement::Continue)),
            Token::Return => {
                let start = self.start_then_advance();
                let expression = self.expression_node()?;
                Ok(self.node_at(start, expression.end, JumpStatement::Return(expression)))
            }
            t => Err(Error::ExpectedOneOfButGot {
                expected_tokens: vec![Token::Break, Token::Continue, Token::Return],
                token: t,
                raw: self.current_slice.into(),
                span: self.current_span.clone(),
            }),
        };
        self.eat(Token::Semicolon);
        res
    }

    fn delete_statement(&mut self) -> Result<StatementNode<'ast>> {
        let start = self.start_then_advance();
        let deleted = self.identifier_node()?;
        self.eat(Token::Semicolon);
        Ok(self.node_at(start, deleted.end, DeleteStatement { deleted }))
    }

    fn expression_statement(&mut self) -> Result<StatementNode<'ast>> {
        let expr = self.expression_node()?;
        self.eat(Token::Semicolon);
        Ok(self.node_at(expr.start, expr.end, expr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use toolshed::Arena;

    #[test]
    fn test_inner_block_statement() {
        let source = "{ const PI = 3; }";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_if_statement() {
        let source = "if (x == 3) y = 2;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_if_else_statement() {
        let source = "if (x == 3) y = 2;\nelse y = 1;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_for_loop_statement() {
        let source = "for (x in 0..3) { }";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_do_while_loop_statement() {
        let source = "do { x += 3 } while (x < 10)";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_while_loop_statement() {
        let source = "while (i < 10) ++i;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_infinite_loop_statement() {
        let source = "loop { ++i; }";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_break_statement() {
        let source = "break;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_continue_statement() {
        let source = "continue;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_return_statement() {
        let source = "return 3;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_delete_statement() {
        let source = "delete myVar;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_expression_statement() {
        let source = "x = y + 3;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.statement_node().unwrap();

        assert_debug_snapshot!(res);
    }
}
