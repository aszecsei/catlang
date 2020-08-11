use toolshed::list::GrowableList;

use crate::syntax::ast::*;
use crate::syntax::error::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn declaration_node(&mut self) -> Result<DeclarationNode<'ast>> {
        // TODO: Possible attributes before this
        let start = self.loc().0;
        let declarator = self.declarator()?;
        let end = self.last_span.end as u32;
        Ok(self.node_at(
            start,
            end,
            Declaration {
                attributes: GrowableList::new().as_list(),
                declarator,
            },
        ))
    }

    fn declarator(&mut self) -> Result<Declarator<'ast>> {
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
                raw: self.current_slice.into(),
                span: self.current_span.clone(),
            }),
        }
    }

    fn const_declarator(&mut self) -> Result<Declarator<'ast>> {
        let _start = self.start_then_advance();
        let identifier = self.identifier_node()?;
        let type_expression = if self.eat(Token::Colon) {
            Some(self.type_node()?)
        } else {
            None
        };
        self.expect(Token::Assign);
        let expression = self.expression_node()?;
        Ok(ConstantDeclarator {
            identifier,
            type_expression,
            expression,
        }
        .into())
    }

    fn type_declarator(&mut self) -> Result<Declarator<'ast>> {
        let _start = self.start_then_advance();
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
        let _start = self.start_then_advance();
        let identifier = self.identifier_node()?;
        let type_expression = if self.eat(Token::Colon) {
            Some(self.type_node()?)
        } else {
            None
        };
        let expression = if self.eat(Token::Assign) {
            Some(self.expression_node()?)
        } else {
            None
        };
        Ok(VariableDeclarator {
            identifier,
            type_expression,
            expression,
        }
        .into())
    }

    fn function_declarator(&mut self) -> Result<Declarator<'ast>> {
        let _start = self.start_then_advance();
        let function_name = self.identifier_node()?;
        let generic_parameters = GrowableList::new();
        if self.eat(Token::LessThan) {
            while self.current_token != Token::GreaterThan && self.current_token != Token::EndOfFile
            {
                generic_parameters.push(self.arena, self.identifier_node()?);
                self.expect_one_of(&[Token::Comma], &[Token::GreaterThan]);
            }
            self.expect(Token::GreaterThan);
        }
        self.expect(Token::LParen);
        let parameters = self.formal_parameter_list()?;
        self.expect(Token::RParen);
        let return_type = if self.eat(Token::Arrow) {
            match self.current_token {
                Token::LCurlyB => None,
                _ => Some(self.type_node()?),
            }
        } else {
            None
        };
        let block = self.block_node()?;

        Ok(FunctionDeclarator {
            function_name,
            generic_parameters: generic_parameters.as_list(),
            parameters,
            return_type,
            block,
        }
        .into())
    }

    fn formal_parameter_list(&mut self) -> Result<NodeList<'ast, Parameter<'ast>>> {
        let param_list = GrowableList::new();
        while self.current_token != Token::RParen && self.current_token != Token::EndOfFile {
            let identifier = self.identifier_node()?;
            self.expect(Token::Colon);
            let type_expression = self.type_node()?;
            param_list.push(
                self.arena,
                self.node_at(
                    identifier.start,
                    type_expression.end,
                    Parameter {
                        identifier,
                        type_expression,
                    },
                ),
            );
            self.expect_one_of(&[Token::Comma], &[Token::RParen]);
        }
        Ok(param_list.as_list())
    }

    fn struct_declarator(&mut self) -> Result<Declarator<'ast>> {
        let _start = self.start_then_advance();
        let identifier = self.identifier_node()?;
        let generic_parameters = GrowableList::new();
        if self.eat(Token::LessThan) {
            while self.current_token != Token::GreaterThan {
                generic_parameters.push(self.arena, self.identifier_node()?);
                self.expect_one_of(&[Token::Comma], &[Token::GreaterThan]);
            }
        }
        self.expect(Token::LCurlyB);
        let members = self.struct_member_list()?;
        self.expect(Token::RCurlyB);
        Ok(StructDeclarator {
            identifier,
            generic_parameters: generic_parameters.as_list(),
            members,
        }
        .into())
    }

    fn struct_member_list(&mut self) -> Result<NodeList<'ast, StructMember<'ast>>> {
        let member_list = GrowableList::new();
        while self.current_token != Token::RCurlyB && self.current_token != Token::EndOfFile {
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
        let representation = if self.eat(Token::Colon) {
            Some(self.enum_representation()?)
        } else {
            None
        };
        self.expect(Token::LCurlyB);
        let values = self.enum_member_list()?;
        self.expect(Token::RCurlyB);

        Ok(EnumDeclarator {
            identifier,
            representation,
            values,
        }
        .into())
    }

    fn enum_representation(&mut self) -> Result<EnumRepresentationType> {
        let tok = self.current_token;
        self.bump();
        match tok {
            Token::S8 => Ok(EnumRepresentationType::S8),
            Token::U8 => Ok(EnumRepresentationType::U8),
            Token::S16 => Ok(EnumRepresentationType::S16),
            Token::U16 => Ok(EnumRepresentationType::U16),
            Token::S32 => Ok(EnumRepresentationType::S32),
            Token::U32 => Ok(EnumRepresentationType::U32),
            Token::S64 => Ok(EnumRepresentationType::S64),
            Token::U64 => Ok(EnumRepresentationType::U64),
            _ => Err(Error::ExpectedOneOfButGot {
                expected_tokens: vec![
                    Token::S8,
                    Token::U8,
                    Token::S16,
                    Token::U16,
                    Token::S32,
                    Token::U32,
                    Token::S64,
                    Token::U64,
                ],
                token: self.current_token,
                raw: self.lexer.slice().into(),
                span: self.lexer.span(),
            }),
        }
    }

    fn enum_member_list(&mut self) -> Result<NodeList<'ast, EnumValue<'ast>>> {
        let member_list = GrowableList::new();
        while self.current_token != Token::RCurlyB && self.current_token != Token::EndOfFile {
            let identifier = self.identifier_node()?;
            let mut end = identifier.end;
            let value = if self.eat(Token::Assign) {
                let expr = self.expression_node()?;
                end = expr.end;
                Some(expr)
            } else {
                None
            };
            member_list.push(
                self.arena,
                self.node_at(identifier.start, end, EnumValue { identifier, value }),
            );
            self.expect_one_of(&[Token::Comma], &[Token::RCurlyB]);
        }

        Ok(member_list.as_list())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use toolshed::Arena;

    #[test]
    fn test_const_declarator() {
        let source = "const PI = 3;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_uninitialized_var_declarator() {
        let source = "let pi;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_initialized_var_declarator() {
        let source = "let pi = 3;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_type_declarator() {
        let source = "type f_array = []float";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_function_declarator() {
        let source = "function foo() {}";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_function_params_declarator() {
        let source = "function foo(i: int) {}";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_function_params_return_declarator() {
        let source = "function foo(i: int) -> int {}";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_function_params_return_generic_declarator() {
        let source = "function foo<T>(i: T) -> int {}";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_struct_declarator() {
        let source = "struct Vector2 { x: f32 = 0; y: f32 = 0; }";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_generic_struct_declarator() {
        let source = "struct Vector2<T> { x: T; y: T; }";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_enum_declarator() {
        let source = "enum MyEnum { A, B }";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_enum_values_declarator() {
        let source = "enum MyEnum { A, B = 3 }";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_enum_repr_declarator() {
        let source = "enum MyEnum : u8 { A, B = 3 }";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.declaration_node().unwrap();

        assert_debug_snapshot!(res);
    }
}
