use toolshed::list::GrowableList;

use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn declaration_node(&mut self) -> Result<DeclarationNode<'ast>> {
        let attr_list = GrowableList::new();
        while self.current_token == Token::LAttr {
            attr_list.push(self.arena, self.attribute_node()?)
        }
        match self.current_token {
            Token::Export => {
                let start = self.start_then_advance();
                let d = self.declarator_node()?;
                let end = self.loc().1;
                Ok(self.node_at(
                    start,
                    end,
                    Declaration {
                        attributes: attr_list.as_list(),
                        declarator: d,
                        is_exported: true,
                    },
                ))
            }
            _ => {
                let start = self.loc().0;
                let d = self.declarator_node()?;
                let end = self.loc().1;
                Ok(self.node_at(
                    start,
                    end,
                    Declaration {
                        attributes: attr_list.as_list(),
                        declarator: d,
                        is_exported: false,
                    },
                ))
            }
        }
    }

    fn attribute_node(&mut self) -> Result<AttributeNode<'ast>> {
        let start = self.start_then_advance();
        let ident = self.identifier_node()?;
        let params = GrowableList::new();
        if self.eat(Token::LParen) {
            while !self.eat(Token::LParen) {
                params.push(self.arena, self.expression_node()?);
                self.expect_one_of(&[Token::Comma], &[Token::LParen]);
            }
        }
        let end = self.loc().1;
        Ok(self.node_at(
            start,
            end,
            Attribute {
                identifier: ident,
                parameters: params.as_list(),
            },
        ))
    }

    fn declarator_node(&mut self) -> Result<Declarator<'ast>> {
        match self.current_token {
            Token::Const => self.const_declarator(),
            Token::Type => self.type_declarator(),
            Token::Let => self.variable_declarator(),
            Token::Function => self.function_declarator(),
            Token::Struct => self.struct_declarator(),
            Token::Enum => self.enum_declarator(),
            _ => Err(Error::ExpectedOneOfButGot {
                expected_tokens: vec![
                    Token::Const,
                    Token::Type,
                    Token::Let,
                    Token::Function,
                    Token::Struct,
                    Token::Enum,
                ],
                token: self.current_token,
                raw: self.lexer.slice().into(),
                span: self.lexer.span(),
            }),
        }
    }

    fn const_declarator(&mut self) -> Result<Declarator<'ast>> {
        Err(Error::NotImplementedError)
    }

    fn type_declarator(&mut self) -> Result<Declarator<'ast>> {
        Err(Error::NotImplementedError)
    }

    fn variable_declarator(&mut self) -> Result<Declarator<'ast>> {
        Err(Error::NotImplementedError)
    }

    fn function_declarator(&mut self) -> Result<Declarator<'ast>> {
        Err(Error::NotImplementedError)
    }

    fn struct_declarator(&mut self) -> Result<Declarator<'ast>> {
        Err(Error::NotImplementedError)
    }

    fn enum_declarator(&mut self) -> Result<Declarator<'ast>> {
        Err(Error::NotImplementedError)
    }
}
