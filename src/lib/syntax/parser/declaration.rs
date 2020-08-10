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
        let is_exported = self.eat(Token::Export);
        let start = self.current_span.start as u32;
        let d = self.declarator_node()?;
        let end = self.last_span.end as u32;
        Ok(self.node_at(
            start,
            end,
            Declaration {
                attributes: attr_list.as_list(),
                declarator: d,
                is_exported,
            },
        ))
    }

    fn attribute_node(&mut self) -> Result<AttributeNode<'ast>> {
        let start = self.start_then_advance();
        let ident = if self.current_token == Token::SOA {
            self.soa_node()?
        } else {
            self.identifier_node()?
        };
        let params = GrowableList::new();
        if self.eat(Token::LParen) {
            while !self.eat(Token::LParen) {
                params.push(self.arena, self.expression_node()?);
                self.expect_one_of(&[Token::Comma], &[Token::LParen]);
            }
        }
        self.expect(Token::RSquareB);
        let end = self.last_span.end as u32;
        Ok(self.node_at(
            start,
            end,
            Attribute {
                identifier: ident,
                parameters: params.as_list(),
            },
        ))
    }

    pub fn soa_node(&mut self) -> Result<IdentifierNode<'ast>> {
        let (start, end) = (self.current_span.start as u32, self.current_span.end as u32);
        let val = self.current_slice;
        self.expect(Token::SOA);
        Ok(self.node_at(start, end, val))
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
        self.expect(Token::Const);
        let identifier = self.identifier_node()?;
        self.expect(Token::Assign);
        let expression = self.expression_node()?;

        Ok(ConstantDeclarator {
            identifier,
            expression,
        }
        .into())
    }

    fn type_declarator(&mut self) -> Result<Declarator<'ast>> {
        self.expect(Token::Type);
        let identifier = self.identifier_node()?;
        self.expect(Token::Assign);
        let type_expression = self.type_node()?;

        Ok(TypeDeclarator {
            identifier,
            type_expression,
        }
        .into())
    }

    fn variable_declarator(&mut self) -> Result<Declarator<'ast>> {
        self.expect(Token::Let);
        let identifier = self.identifier_node()?;
        let type_expression = match self.current_token {
            Token::Colon => {
                self.bump();
                Some(self.type_node()?)
            }
            _ => None,
        };
        let expression = match self.current_token {
            Token::Assign => {
                self.bump();
                Some(self.expression_node()?)
            }
            _ => None,
        };

        Ok(VariableDeclarator {
            identifier,
            type_expression,
            expression,
        }
        .into())
    }

    fn function_declarator(&mut self) -> Result<Declarator<'ast>> {
        self.expect(Token::Function);
        let identifier = self.identifier_node()?;
        self.expect(Token::LParen);
        let parameters = self.formal_parameter_list()?;
        self.expect(Token::RParen);

        let return_type = match self.current_token {
            Token::LCurlyB => None,
            Token::Arrow if self.peek_token == Token::LCurlyB => {
                self.bump();
                None
            }
            Token::Arrow => {
                self.bump();
                Some(self.type_node()?)
            }
            token => return Err(Error::ExpectedFunctionButGot { token }),
        };
        self.expect(Token::LCurlyB);
        let block = self.block_node();
        self.expect(Token::RCurlyB);

        Ok(FunctionDeclarator {
            identifier,
            parameters,
            return_type,
            block,
        }
        .into())
    }

    fn formal_parameter_list(&mut self) -> Result<NodeList<'ast, Parameter<'ast>>> {
        let param_list = GrowableList::new();
        while self.current_token != Token::RParen {
            let start = self.current_span.start as u32;
            let is_const = self.eat(Token::Const);
            let identifier = self.identifier_node()?;
            self.expect(Token::Colon);
            let type_expression = self.type_node()?;
            let end = type_expression.end;
            param_list.push(
                self.arena,
                self.node_at(
                    start,
                    end,
                    Parameter {
                        identifier,
                        is_const,
                        type_expression,
                    },
                ),
            );
            self.expect_one_of(&[Token::Comma], &[Token::RParen]);
        }

        Ok(param_list.as_list())
    }

    fn struct_declarator(&mut self) -> Result<Declarator<'ast>> {
        self.expect(Token::Struct);
        let identifier = self.identifier_node()?;
        self.expect(Token::LCurlyB);
        let members = self.struct_member_list()?;
        self.expect(Token::RCurlyB);

        Ok(StructDeclarator {
            identifier,
            members,
        }
        .into())
    }

    fn struct_member_list(&mut self) -> Result<NodeList<'ast, StructMember<'ast>>> {
        let member_list = GrowableList::new();
        while self.current_token != Token::RCurlyB {
            let start = self.current_span.start as u32;
            let is_owned = self.eat(Token::Owned);
            let identifier = self.identifier_node()?;
            self.expect(Token::Colon);
            let type_expression = self.type_node()?;
            let mut end = type_expression.end;
            let default_value = if self.eat(Token::Assign) {
                let expr = self.expression_node()?;
                end = expr.end;
                Some(expr)
            } else {
                None
            };
            member_list.push(
                self.arena,
                self.node_at(
                    start,
                    end,
                    StructMember {
                        identifier,
                        is_owned,
                        type_expression,
                        default_value,
                    },
                ),
            );
            self.expect_one_of(&[Token::Semicolon], &[Token::RCurlyB]);
        }

        Ok(member_list.as_list())
    }

    fn enum_declarator(&mut self) -> Result<Declarator<'ast>> {
        self.expect(Token::Enum);
        let identifier = self.identifier_node()?;
        let enum_type = if self.eat(Token::Colon) {
            Some(self.type_node()?)
        } else {
            None
        };
        self.expect(Token::LCurlyB);
        let values = GrowableList::new();
        while self.current_token != Token::RCurlyB {
            values.push(self.arena, self.identifier_node()?);
            self.expect_one_of(&[Token::Comma], &[Token::RCurlyB]);
        }
        self.expect(Token::RCurlyB);

        Ok(EnumDeclarator {
            identifier,
            type_expression: enum_type,
            values: values.as_list(),
        }
        .into())
    }
}
