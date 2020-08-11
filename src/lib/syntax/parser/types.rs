use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;
use toolshed::list::GrowableList;

impl<'ast> Parser<'ast> {
    pub fn type_node(&mut self) -> Result<TypeExpressionNode<'ast>> {
        let mut lhs = self.unary_type()?;
        while let Token::BitOr = self.current_token {
            self.bump();
            let rhs = self.unary_type()?;
            lhs = self.node_at(
                lhs.start,
                rhs.end,
                BinaryTypeExpression {
                    op: BinaryTypeOperator::TypeUnion,
                    left: lhs,
                    right: rhs,
                },
            );
        }
        Ok(lhs)
    }

    fn unary_type(&mut self) -> Result<TypeExpressionNode<'ast>> {
        match self.current_token {
            Token::Mul => {
                // Pointer-to
                let start = self.start_then_advance();
                let inner = self.unary_type()?;
                Ok(self.node_at(
                    start,
                    inner.end,
                    UnaryTypeExpression {
                        op: UnaryTypeOperator::PointerTo,
                        inner,
                    },
                ))
            }
            Token::LSquareB => {
                let start = self.start_then_advance();
                match self.current_token {
                    Token::RSquareB => {
                        // statically-sized type
                        self.bump();
                        let inner = self.unary_type()?;
                        Ok(self.node_at(
                            start,
                            inner.end,
                            UnaryTypeExpression {
                                op: UnaryTypeOperator::SizedArray,
                                inner,
                            },
                        ))
                    }
                    Token::DotDot => {
                        // dynamically-sized type
                        self.bump();
                        self.expect(Token::RSquareB);
                        let inner = self.unary_type()?;
                        Ok(self.node_at(
                            start,
                            inner.end,
                            UnaryTypeExpression {
                                op: UnaryTypeOperator::UnsizedArray,
                                inner,
                            },
                        ))
                    }
                    _ => unimplemented!(),
                }
            }
            Token::Const => {
                let start = self.start_then_advance();
                let inner = self.unary_type()?;
                Ok(self.node_at(
                    start,
                    inner.end,
                    UnaryTypeExpression {
                        op: UnaryTypeOperator::Const,
                        inner,
                    },
                ))
            }
            Token::Volatile => {
                let start = self.start_then_advance();
                let inner = self.unary_type()?;
                Ok(self.node_at(
                    start,
                    inner.end,
                    UnaryTypeExpression {
                        op: UnaryTypeOperator::Volatile,
                        inner,
                    },
                ))
            }
            Token::Question => {
                let start = self.start_then_advance();
                let inner = self.unary_type()?;
                Ok(self.node_at(
                    start,
                    inner.end,
                    UnaryTypeExpression {
                        op: UnaryTypeOperator::Optional,
                        inner,
                    },
                ))
            }
            _ => self.simple_type_expression(),
        }
    }

    fn simple_type_expression(&mut self) -> Result<TypeExpressionNode<'ast>> {
        match self.current_token {
            Token::Typeof => {
                // typeof [expression]
                let start = self.start_then_advance();
                let expression = self.expression_node()?;
                Ok(self.node_at(
                    start,
                    expression.end,
                    SimpleTypeExpression::Typeof(expression),
                ))
            }
            Token::LParen => {
                // sub-expression
                let start = self.start_then_advance();
                let inner = self.type_node()?;
                let end = self.expect_end(Token::RParen);
                Ok(self.node_at(start, end, SimpleTypeExpression::SubExpression(inner)))
            }
            Token::Any => Ok(self.node_at_token(SimpleTypeExpression::Any)),
            // primitives
            Token::S8 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::S8)))
            }
            Token::U8 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::U8)))
            }
            Token::S16 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::S16)))
            }
            Token::U16 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::U16)))
            }
            Token::S32 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::S32)))
            }
            Token::U32 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::U32)))
            }
            Token::S64 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::S64)))
            }
            Token::U64 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::U64)))
            }
            Token::Char => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::Char)))
            }
            Token::Short => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::Short)))
            }
            Token::Int => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::Int)))
            }
            Token::Long => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::Long)))
            }
            Token::CShort => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::CShort)))
            }
            Token::CUShort => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::CUShort)))
            }
            Token::CInt => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::CInt)))
            }
            Token::CUInt => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::CUInt)))
            }
            Token::CLong => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::CLong)))
            }
            Token::CULong => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::CULong)))
            }
            Token::CLongLong => Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(
                PrimitiveType::CLongLong,
            ))),
            Token::CULongLong => Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(
                PrimitiveType::CULongLong,
            ))),
            Token::CLongDouble => Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(
                PrimitiveType::CLongDouble,
            ))),
            Token::BoolType => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::Bool)))
            }
            Token::F32 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::F32)))
            }
            Token::F64 => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::F64)))
            }
            Token::Float => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::Float)))
            }
            Token::Double => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::Double)))
            }
            Token::Null => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::Null)))
            }
            Token::NoReturn => Ok(
                self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::NoReturn))
            ),
            Token::CVoid => {
                Ok(self.node_at_token(SimpleTypeExpression::PrimitiveType(PrimitiveType::CVoid)))
            }
            // named type
            _ => {
                let identifier = self.identifier_node()?;
                let mut end = identifier.end;
                let generic_parameters = GrowableList::new();
                if self.eat(Token::LessThan) {
                    while self.current_token != Token::GreaterThan
                        && self.current_token != Token::EndOfFile
                    {
                        generic_parameters.push(self.arena, self.type_node()?);
                    }
                    end = self.expect_end(Token::GreaterThan);
                }
                Ok(self.node_at(
                    identifier.start,
                    end,
                    SimpleTypeExpression::NamedType(NamedType {
                        identifier,
                        generic_parameters: generic_parameters.as_list(),
                    }),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use toolshed::Arena;

    #[test]
    fn test_primitive_type() {
        let source = "float";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.type_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_optional_type() {
        let source = "?float";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.type_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_sized_array_type() {
        let source = "[]float";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.type_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_unsized_array_type() {
        let source = "[..]float";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.type_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_const_ptr_type() {
        let source = "*const float";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.type_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_named_type() {
        let source = "string";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.type_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_named_generic_type() {
        let source = "Array<float>";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.type_node().unwrap();

        assert_debug_snapshot!(res);
    }
}
