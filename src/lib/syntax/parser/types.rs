use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn type_node(&mut self) -> Result<TypeExpressionNode<'ast>> {
        let mut type_expr = self.unary_type()?;
        loop {
            match self.current_token {
                Token::BitOr => {
                    self.bump();
                    let rhs = self.unary_type()?;
                    type_expr = self.node_at(
                        type_expr.start,
                        rhs.end,
                        TypeUnionExpression {
                            left: type_expr,
                            right: rhs,
                        },
                    );
                }
                _ => break Ok(type_expr),
            }
        }
    }

    fn unary_type(&mut self) -> Result<TypeExpressionNode<'ast>> {
        let mut type_expr: TypeExpressionNode<'ast> = match self.current_token {
            Token::Mul => {
                let start = self.start_then_advance();
                let inner = self.unary_type()?;
                self.node_at(start, inner.end, TypeExpression::PointerType(inner))
            }
            Token::Typeof => {
                let start = self.start_then_advance();
                let inner = self.expression_node()?;
                self.node_at(start, inner.end, inner)
            }
            Token::LParen => {
                self.bump();
                let inner = self.type_node()?;
                self.expect(Token::RParen);
                inner
            }
            Token::LSquareB if self.peek_token == Token::DotDot => {
                let start = self.start_then_advance();
                self.expect(Token::DotDot);
                self.expect(Token::RSquareB);
                let inner = self.type_node()?;
                self.node_at(start, inner.end, TypeExpression::UnsizedArrayType(inner))
            }
            Token::LSquareB => {
                let start = self.start_then_advance();
                let size = self.expression_node()?;
                self.expect(Token::RSquareB);
                let inner = self.type_node()?;
                self.node_at(start, inner.end, SizedArrayType { size, inner })
            }
            _ => {
                let (start, end) = self.loc();
                let inner = self.primitive_type()?;
                self.node_at(start, end, inner)
            }
        };
        loop {
            match self.current_token {
                Token::Question => {
                    let end = self.end_then_advance();
                    type_expr = self.node_at(
                        type_expr.start,
                        end,
                        TypeExpression::OptionalType(type_expr),
                    );
                }
                _ => break Ok(type_expr),
            }
        }
    }

    fn primitive_type(&mut self) -> Result<PrimitiveType<'ast>> {
        match self.current_token {
            Token::S8 => {
                self.bump();
                Ok(PrimitiveType::S8)
            }
            Token::U8 => {
                self.bump();
                Ok(PrimitiveType::U8)
            }
            Token::S16 => {
                self.bump();
                Ok(PrimitiveType::S16)
            }
            Token::U16 => {
                self.bump();
                Ok(PrimitiveType::U16)
            }
            Token::S32 => {
                self.bump();
                Ok(PrimitiveType::S32)
            }
            Token::U32 => {
                self.bump();
                Ok(PrimitiveType::U32)
            }
            Token::S64 => {
                self.bump();
                Ok(PrimitiveType::S64)
            }
            Token::U64 => {
                self.bump();
                Ok(PrimitiveType::U64)
            }
            Token::Char => {
                self.bump();
                Ok(PrimitiveType::U8)
            }
            Token::Short => {
                self.bump();
                Ok(PrimitiveType::S16)
            }
            Token::Int => {
                self.bump();
                Ok(PrimitiveType::S32)
            }
            Token::Long => {
                self.bump();
                Ok(PrimitiveType::S64)
            }
            Token::BoolType => {
                self.bump();
                Ok(PrimitiveType::Bool)
            }
            Token::Float => {
                self.bump();
                Ok(PrimitiveType::Float)
            }
            Token::Double => {
                self.bump();
                Ok(PrimitiveType::Double)
            }
            Token::Null => {
                self.bump();
                Ok(PrimitiveType::Null)
            }
            _ => Ok(self.identifier_node()?.into()),
        }
    }
}
