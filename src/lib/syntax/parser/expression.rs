use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

// Pratt parsing! Note that if the return of .0 < .1, the operator will be left-associative,
// and if .0 > .1, the operator will be right-associative.

fn infix_binding_power(tok: Token) -> Option<(u8, u8)> {
    match tok {
        // Assignment operators
        Token::Assign => Some((2, 1)),
        Token::AddAssign => Some((2, 1)),
        Token::SubAssign => Some((2, 1)),
        Token::MulAssign => Some((2, 1)),
        Token::QuoAssign => Some((2, 1)),
        Token::ModAssign => Some((2, 1)),
        Token::BitAndAssign => Some((2, 1)),
        Token::AndAssign => Some((2, 1)),
        Token::BitOrAssign => Some((2, 1)),
        Token::OrAssign => Some((2, 1)),
        Token::XorAssign => Some((2, 1)),
        Token::ShiftLAssign => Some((2, 1)),
        Token::ShiftRAssign => Some((2, 1)),
        Token::NullCoalesceAssign => Some((2, 1)),
        // Ternary
        Token::Question => Some((4, 3)),
        // Logical or
        Token::Or => Some((5, 6)),
        // Logical and
        Token::And => Some((7, 8)),
        // Equality
        Token::Equals | Token::NotEquals => Some((9, 10)),
        // Type test
        Token::Is => Some((11, 12)),
        // Comparison
        Token::LessThan => Some((13, 14)),
        Token::LessThanEquals => Some((13, 14)),
        Token::GreaterThan => Some((13, 14)),
        Token::GreaterThanEquals => Some((13, 14)),
        Token::In => Some((13, 14)),
        // Bitwise or
        Token::BitOr => Some((15, 16)),
        // Bitwise xor
        Token::Xor => Some((17, 18)),
        // Bitwise and
        Token::BitAnd => Some((19, 20)),
        // Bitshift
        Token::ShiftL | Token::ShiftR => Some((21, 22)),
        // Range
        Token::DotDot | Token::DotDotDot => Some((23, 24)),
        // Add, subtract
        Token::Add | Token::Sub => Some((25, 26)),
        // Multiply, divide, modulo
        Token::Mul | Token::Quo | Token::Mod => Some((27, 28)),
        // Casting, null coalesce
        Token::As | Token::NullCoalesce => Some((30, 29)),
        _ => None,
    }
}

fn prefix_binding_power(tok: Token) -> Option<((), u8)> {
    match tok {
        Token::Increment => Some(((), 29)),
        Token::Decrement => Some(((), 29)),
        Token::Add => Some(((), 29)),
        Token::Sub => Some(((), 29)),
        Token::Not => Some(((), 29)),
        Token::BitNot => Some(((), 29)),
        Token::At => Some(((), 29)),
        Token::Mul => Some(((), 29)),
        _ => None,
    }
}

fn postfix_binding_power(tok: Token) -> Option<(u8, ())> {
    match tok {
        Token::Increment => Some((31, ())),
        Token::Decrement => Some((31, ())),
        Token::Not => Some((31, ())),
        Token::LParen => Some((31, ())),
        Token::LSquareB => Some((31, ())),
        Token::Dot => Some((31, ())),
        Token::NullConditional => Some((31, ())),
        Token::NullConditionalIndex => Some((31, ())),
        _ => None,
    }
}

impl<'ast> Parser<'ast> {
    pub fn expression_node(&mut self) -> Result<ExpressionNode<'ast>> {
        self.expression_bp(0)
    }

    fn expression_bp(&mut self, min_bp: u8) -> Result<ExpressionNode<'ast>> {
        let mut lhs: ExpressionNode<'ast> = match self.current_token {
            Token::Ident => {
                let identifier = self.identifier_node()?;
                self.node_at(identifier.start, identifier.end, identifier)
            }
            Token::Integer(_) => self.node_from_slice(Primitive::DecimalNumber),
            Token::Bool(b) => self.node_at_token(Primitive::Bool(b)),
            Token::LParen => {
                self.bump();
                let lhs = self.expression_bp(0)?;
                self.expect(Token::RParen);
                lhs
            }
            t => match prefix_binding_power(t) {
                Some(((), r_bp)) => {
                    let start = self.start_then_advance();
                    let rhs = self.expression_bp(r_bp)?;
                    self.node_at(
                        start,
                        rhs.end,
                        PrefixExpression {
                            operand: rhs,
                            operator: t.into(),
                        },
                    )
                }
                None => unimplemented!(),
            },
        };

        loop {
            let op = match self.current_token {
                Token::EndOfFile => break,
                t => t,
            };

            if let Some((l_bp, ())) = postfix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }
                self.bump();

                lhs = match op {
                    Token::Increment => self.node_at_token(PostfixExpression {
                        operand: lhs,
                        operator: PostfixOperator::Increment,
                    }),
                    Token::Decrement => self.node_at_token(PostfixExpression {
                        operand: lhs,
                        operator: PostfixOperator::Decrement,
                    }),
                    Token::Not => self.node_at_token(PostfixExpression {
                        operand: lhs,
                        operator: PostfixOperator::NullForgiving,
                    }),
                    Token::LParen => unimplemented!(), // function call
                    Token::LSquareB => unimplemented!(), // index
                    Token::Dot => unimplemented!(),    // member access
                    Token::NullConditional => self.node_at_token(PostfixExpression {
                        operand: lhs,
                        operator: PostfixOperator::NullConditional,
                    }),
                    Token::NullConditionalIndex => self.node_at_token(PostfixExpression {
                        operand: lhs,
                        operator: PostfixOperator::NullConditionalIndex,
                    }),
                    _ => unimplemented!(),
                };
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }
                self.bump();

                lhs = match op {
                    // Assignment operators
                    Token::Assign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::Plain,
                                right: rhs,
                            },
                        )
                    }
                    Token::AddAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::Addition,
                                right: rhs,
                            },
                        )
                    }
                    Token::SubAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::Subtraction,
                                right: rhs,
                            },
                        )
                    }
                    Token::MulAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::Multiplication,
                                right: rhs,
                            },
                        )
                    }
                    Token::QuoAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::Division,
                                right: rhs,
                            },
                        )
                    }
                    Token::ModAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::Remainder,
                                right: rhs,
                            },
                        )
                    }
                    Token::BitAndAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::BitAnd,
                                right: rhs,
                            },
                        )
                    }
                    Token::AndAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::LogicalAnd,
                                right: rhs,
                            },
                        )
                    }
                    Token::BitOrAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::BitOr,
                                right: rhs,
                            },
                        )
                    }
                    Token::OrAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::LogicalOr,
                                right: rhs,
                            },
                        )
                    }
                    Token::XorAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::BitXor,
                                right: rhs,
                            },
                        )
                    }
                    Token::ShiftLAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::BitShiftLeft,
                                right: rhs,
                            },
                        )
                    }
                    Token::ShiftRAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::BitShiftRight,
                                right: rhs,
                            },
                        )
                    }
                    Token::NullCoalesceAssign => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            AssignmentExpression {
                                left: lhs,
                                operator: AssignmentOperator::NullCoalesce,
                                right: rhs,
                            },
                        )
                    }
                    // Ternary
                    Token::Question => {
                        let mhs = self.expression_bp(0)?;
                        self.expect(Token::Colon);
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            TernaryExpression {
                                condition: lhs,
                                when_true: mhs,
                                when_false: rhs,
                            },
                        )
                    }
                    // Logical or
                    Token::Or => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::LogicalOr,
                                right: rhs,
                            },
                        )
                    }
                    // Logical and
                    Token::And => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::LogicalAnd,
                                right: rhs,
                            },
                        )
                    }
                    // Equality
                    Token::Equals => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::Equals,
                                right: rhs,
                            },
                        )
                    }
                    Token::NotEquals => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::NotEquals,
                                right: rhs,
                            },
                        )
                    }
                    // Type test
                    Token::Is => unimplemented!(),
                    // Comparison
                    Token::LessThan => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::LessThan,
                                right: rhs,
                            },
                        )
                    }
                    Token::LessThanEquals => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::LessThanEquals,
                                right: rhs,
                            },
                        )
                    }
                    Token::GreaterThan => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::GreaterThan,
                                right: rhs,
                            },
                        )
                    }
                    Token::GreaterThanEquals => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::GreaterThanEquals,
                                right: rhs,
                            },
                        )
                    }
                    Token::In => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::In,
                                right: rhs,
                            },
                        )
                    }
                    // Bitwise or
                    Token::BitOr => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::BitOr,
                                right: rhs,
                            },
                        )
                    }
                    // Bitwise xor
                    Token::Xor => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::BitXor,
                                right: rhs,
                            },
                        )
                    }
                    // Bitwise and
                    Token::BitAnd => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::BitAnd,
                                right: rhs,
                            },
                        )
                    }
                    // Bitshift
                    Token::ShiftL => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::BitShiftLeft,
                                right: rhs,
                            },
                        )
                    }
                    Token::ShiftR => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::BitShiftRight,
                                right: rhs,
                            },
                        )
                    }
                    // Range
                    Token::DotDot => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::RangeExclusive,
                                right: rhs,
                            },
                        )
                    }
                    Token::DotDotDot => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::RangeInclusive,
                                right: rhs,
                            },
                        )
                    }
                    // Add, subtract
                    Token::Add => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::Addition,
                                right: rhs,
                            },
                        )
                    }
                    Token::Sub => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::Subtraction,
                                right: rhs,
                            },
                        )
                    }
                    // Multiply, divide, modulo
                    Token::Mul => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::Multiplication,
                                right: rhs,
                            },
                        )
                    }
                    Token::Quo => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::Division,
                                right: rhs,
                            },
                        )
                    }
                    Token::Mod => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::Remainder,
                                right: rhs,
                            },
                        )
                    }
                    // Casting, null coalesce
                    Token::As => {
                        let operator = if self.eat(Token::Not) {
                            BinaryOperator::ForcedCast
                        } else {
                            BinaryOperator::Cast
                        };
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator,
                                right: rhs,
                            },
                        )
                    }
                    Token::NullCoalesce => {
                        let rhs = self.expression_bp(r_bp)?;
                        self.node_at(
                            lhs.start,
                            rhs.end,
                            BinaryExpression {
                                left: lhs,
                                operator: BinaryOperator::NullCoalesce,
                                right: rhs,
                            },
                        )
                    }
                    _ => unimplemented!(),
                };
                continue;
            }
            break;
        }

        Ok(lhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use toolshed::Arena;

    #[test]
    fn test_basic_expression() {
        let source = "(1 + 2) * 3";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.expression_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_ternary_associativity() {
        let source = "1 == 1 ? 2 : 3 == 4 ? 5 : 6";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.expression_node().unwrap();

        assert_debug_snapshot!(res);
    }
}
