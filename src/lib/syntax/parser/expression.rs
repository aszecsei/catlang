use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn expression_node(&mut self) -> Result<ExpressionNode<'ast>> {
        self.assignment_expression()
    }

    // Right-associative
    fn assignment_expression(&mut self) -> Result<ExpressionNode<'ast>> {
        let lhs = self.conditional_expression()?;
        let assignment = match self.current_token {
            Token::Assign => Some(AssignmentOperator::Plain),
            Token::AddAssign => Some(AssignmentOperator::Addition),
            Token::SubAssign => Some(AssignmentOperator::Subtraction),
            Token::MulAssign => Some(AssignmentOperator::Multiplication),
            Token::QuoAssign => Some(AssignmentOperator::Division),
            Token::ModAssign => Some(AssignmentOperator::Remainder),
            Token::BitOrAssign => Some(AssignmentOperator::BitOr),
            Token::OrAssign => Some(AssignmentOperator::LogicalOr),
            Token::BitAndAssign => Some(AssignmentOperator::BitAnd),
            Token::AndAssign => Some(AssignmentOperator::LogicalAnd),
            Token::XorAssign => Some(AssignmentOperator::BitXor),
            Token::ShiftLAssign => Some(AssignmentOperator::BitShiftLeft),
            Token::ShiftRAssign => Some(AssignmentOperator::BitShiftRight),
            Token::NullCoalesceAssign => Some(AssignmentOperator::NullCoalesce),
            _ => None,
        };
        if let Some(assignment_operator) = assignment {
            self.bump();
            let rhs = self.expression_node()?;
            Ok(self.node_at(
                lhs.start,
                rhs.end,
                AssignmentExpression {
                    left: lhs,
                    operator: assignment_operator,
                    right: rhs,
                },
            ))
        } else {
            Ok(lhs)
        }
    }

    // Right-associative
    fn conditional_expression(&mut self) -> Result<ExpressionNode<'ast>> {
        let condition = self.comparing_expression()?;
        if self.eat(Token::Question) {
            let true_value = self.expression_node()?;
            self.expect(Token::Colon);
            let false_value = self.expression_node()?;
            Ok(self.node_at(
                condition.start,
                false_value.end,
                TernaryExpression {
                    condition,
                    when_true: true_value,
                    when_false: false_value,
                },
            ))
        } else {
            Ok(condition)
        }
    }

    // Right-associative
    fn comparing_expression(&mut self) -> Result<ExpressionNode<'ast>> {
        let lhs = self.comparand()?;
        let operator = match self.current_token {
            Token::Equals => Some(BinaryOperator::Equals),
            Token::GreaterThan => Some(BinaryOperator::GreaterThan),
            Token::GreaterThanEquals => Some(BinaryOperator::GreaterThanEquals),
            Token::NotEquals => Some(BinaryOperator::NotEquals),
            Token::LessThan => Some(BinaryOperator::LessThan),
            Token::LessThanEquals => Some(BinaryOperator::LessThanEquals),
            Token::In => Some(BinaryOperator::In),
            _ => None,
        };
        if let Some(binary_operator) = operator {
            self.bump();
            let rhs = self.expression_node()?;
            Ok(self.node_at(
                lhs.start,
                rhs.end,
                BinaryExpression {
                    left: lhs,
                    operator: binary_operator,
                    right: rhs,
                },
            ))
        } else {
            Ok(lhs)
        }
    }

    // Left-associative
    fn comparand(&mut self) -> Result<ExpressionNode<'ast>> {
        let lhs = self.term()?;
        let mut expr = lhs;
        loop {
            let operator = match self.current_token {
                Token::Add => Some(BinaryOperator::Addition),
                Token::Sub => Some(BinaryOperator::Subtraction),
                Token::Or => Some(BinaryOperator::LogicalOr),
                Token::BitOr => Some(BinaryOperator::BitOr),
                _ => None,
            };
            if let Some(binary_operator) = operator {
                self.bump();
                let rhs = self.term()?;
                expr = self.node_at(
                    expr.start,
                    expr.end,
                    BinaryExpression {
                        left: lhs,
                        operator: binary_operator,
                        right: rhs,
                    },
                );
            } else {
                break Ok(expr);
            }
        }
    }

    // Left-associative
    fn term(&mut self) -> Result<ExpressionNode<'ast>> {
        let lhs = self.factor()?;
        let mut expr = lhs;
        loop {
            let operator = match self.current_token {
                Token::Mul => Some(BinaryOperator::Multiplication),
                Token::Quo => Some(BinaryOperator::Division),
                Token::And => Some(BinaryOperator::LogicalAnd),
                Token::BitAnd => Some(BinaryOperator::BitAnd),
                _ => None,
            };
            if let Some(binary_operator) = operator {
                self.bump();
                let rhs = self.factor()?;
                expr = self.node_at(
                    expr.start,
                    expr.end,
                    BinaryExpression {
                        left: lhs,
                        operator: binary_operator,
                        right: rhs,
                    },
                );
            } else {
                break Ok(expr);
            }
        }
    }

    // Left-associative
    fn factor(&mut self) -> Result<ExpressionNode<'ast>> {
        let lhs = self.unary()?;
        let mut expr = lhs;
        loop {
            let operator = match self.current_token {
                Token::ShiftL => Some(BinaryOperator::BitShiftLeft),
                Token::ShiftR => Some(BinaryOperator::BitShiftRight),
                _ => None,
            };
            if let Some(binary_operator) = operator {
                self.bump();
                let rhs = self.unary()?;
                expr = self.node_at(
                    expr.start,
                    expr.end,
                    BinaryExpression {
                        left: lhs,
                        operator: binary_operator,
                        right: rhs,
                    },
                );
            } else {
                break Ok(expr);
            }
        }
    }

    fn unary(&mut self) -> Result<ExpressionNode<'ast>> {
        let operator = match self.current_token {
            Token::Not => Some(PrefixOperator::LogicalNot),
            Token::BitNot => Some(PrefixOperator::BitNot),
            Token::Sub => Some(PrefixOperator::Minus),
            Token::Add => Some(PrefixOperator::Plus),
            Token::Increment => Some(PrefixOperator::Increment),
            Token::Decrement => Some(PrefixOperator::Decrement),
            _ => None,
        };
        if let Some(prefix_operator) = operator {
            let start = self.loc().0;
            self.bump();
            let rhs = self.unary()?;
            return Ok(self.node_at(
                start,
                rhs.end,
                PrefixExpression {
                    operator: prefix_operator,
                    operand: rhs,
                },
            ));
        }

        let mut expr = self.primary_expression()?;
        loop {
            let operator = match self.current_token {
                Token::Increment => Some(PostfixOperator::Increment),
                Token::Decrement => Some(PostfixOperator::Decrement),
                Token::Not => Some(PostfixOperator::NullForgiving),
                _ => None,
            };
            if let Some(postfix_operator) = operator {
                let end = self.loc().1;
                self.bump();
                expr = self.node_at(
                    expr.start,
                    end,
                    PostfixExpression {
                        operator: postfix_operator,
                        operand: expr,
                    },
                );
            } else {
                break Ok(expr);
            }
        }
    }

    fn primary_expression(&mut self) -> Result<ExpressionNode<'ast>> {
        match self.current_token {
            Token::Integer(_) => Ok(self.node_from_slice(|s| Primitive::DecimalNumber(s))),
            Token::LiteralString => Ok(self.node_from_slice(|s| Primitive::String(s))),
            Token::Bool(b) => Ok(self.node_at_token(Primitive::Bool(b))),
            Token::Null => Ok(self.node_at_token(Primitive::Null)),
            Token::LParen => {
                self.expect(Token::LParen);
                if self.peek_token == Token::Colon {
                    // TODO: Lambdas
                    Err(Error::NotImplementedError)
                } else {
                    let inside = self.expression_node()?;
                    self.expect(Token::RParen);
                    Ok(inside)
                }
            }
            _ => self.reference(),
        }
    }

    fn reference(&mut self) -> Result<ExpressionNode<'ast>> {
        // TODO: Remove this exception
        #[allow(clippy::match_single_binding)]
        match self.current_token {
            // Token::At => Address-of reference
            // Token::Mul => Dereference
            // TODO: Member access
            // TODO: Function call
            // TODO: Constructor call
            // TODO: Array reference
            // TODO: Cast reference
            _ => {
                let ident = self.identifier_node()?;
                Ok(self.node_at(ident.start, ident.end, ident))
            }
        }
    }
}
