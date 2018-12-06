use std::rc::Rc;
use crate::syntax::ast;
use crate::syntax::lexer;
use crate::syntax::token;

use log::{debug, error};

use crate::syntax::error::Error;
use crate::syntax::context::Context;

pub struct Parser<'a> {
    fname: &'a str,
    errors: Vec<Error>,
    scanner: lexer::StringReader<'a>,
    current_scope: Rc<ast::Scope>,
    top_scope: Rc<ast::Scope>,
}

impl<'a> Parser<'a> {
    pub fn parse_file(fname: &'a str, src: &'a str, context: &'a mut Context) -> ast::Block {
        let mut p = Parser::new(fname, src, None, context);
        let res = p.parse_block();
        // TODO: Kinder error logging (i.e. hide > 10 errors)
        for e in p.errors {
            error!("{}", e.get_msg());
        }
        res
    }

    fn expect(&mut self, tok: token::Token) -> Result<(), Error> {
        let r = {
            let lexeme = self.scanner.token().tok;

            if lexeme != tok {
                Err(Error::new(format!(
                    "Expected '{}' but got '{}'",
                    tok, lexeme
                )))
            } else {
                Ok(())
            }
        };
        self.next();
        r
    }

    fn new(fname: &'a str, src: &'a str, scope: Option<ast::Scope>, context: &'a mut Context) -> Self {
        let scope = match scope {
            Some(s) => s,
            None => ast::Scope::new(None),
        };
        let rc_scope = Rc::new(scope);

        let source_file = {
            let mut source_map = context.get_source_map();
            let source_file = source_map.add_file(String::from(fname), String::from(src));
            source_file
        };

        let mut p = Parser {
            fname,
            errors: vec![],
            scanner: lexer::StringReader::new(context, source_file.clone(), src),
            current_scope: rc_scope.clone(),
            top_scope: rc_scope,
        };
        p.next();
        p
    }

    fn next(&mut self) {
        self.scanner.next();
    }

    fn open_scope(&mut self) {
        self.current_scope = Rc::new(ast::Scope::new(Some(self.current_scope.clone())));
    }

    fn close_scope(&mut self) {
        self.current_scope = self.current_scope.parent.clone().unwrap(); // TODO: Unwrap
    }

    fn is_declaration_starter(t: token::Token) -> bool {
        t == token::Token::Export
            || t == token::Token::Const
            || t == token::Token::Type
            || t == token::Token::Let
            || t == token::Token::Function
            || t == token::Token::Struct
            || t == token::Token::Enum
    }

    fn parse_block(&mut self) -> ast::Block {
        let mut elements = vec![];
        let mut sp = self.scanner.token().sp;
        loop {
            match self.scanner.token().tok {
                token::Token::EOF => break,
                token::Token::RCurlyB => break,
                token::Token::Semicolon => {
                    self.next();
                    continue;
                }
                tok => {
                    debug!("Parsing start of block element {:?}", tok);
                    sp = sp.merge(self.scanner.token().sp);

                    if Parser::is_declaration_starter(tok) {
                        let declaration = self.parse_declaration();
                        match declaration {
                            Err(e) => self.errors.push(e),
                            Ok(d) => elements.push(ast::BlockElement::Declaration(d)),
                        }
                    } else {
                        let statement = self.parse_statement();
                        match statement {
                            Err(e) => self.errors.push(e),
                            Ok(s) => elements.push(ast::BlockElement::Statement(s)),
                        }
                    }
                }
            }
        }
        ast::Block {
            elements,
            span: sp,
        }
    }

    fn parse_declaration(&mut self) -> Result<ast::Declaration, Error> {
        match self.scanner.token().tok {
            token::Token::Export => {
                self.next();
                let d = self.parse_declarator()?;
                Ok(ast::Declaration {
                    is_exported: true,
                    declarator: d,
                })
            }
            _ => {
                let d = self.parse_declarator()?;
                Ok(ast::Declaration {
                    is_exported: false,
                    declarator: d
                })
            }
        }
    }

    fn parse_declarator(&mut self) -> Result<ast::Declarator, Error> {
        match self.scanner.token().tok {
            token::Token::Const => Ok(ast::Declarator::ConstantDeclarator(
                self.parse_const_declarator()?,
            )),
            token::Token::Type => Ok(ast::Declarator::TypeDeclarator(
                self.parse_type_declarator()?,
            )),
            token::Token::Let => Ok(ast::Declarator::VariableDeclarator(
                self.parse_variable_declarator()?,
            )),
            token::Token::Function => Ok(ast::Declarator::FunctionDeclarator(
                self.parse_function_declarator()?,
            )),
            token::Token::Struct => Ok(ast::Declarator::StructDeclarator(
                self.parse_struct_declarator()?,
            )),
            token::Token::Enum => Ok(ast::Declarator::EnumDeclarator(
                self.parse_enum_declarator()?,
            )),
            tok => Err(Error::new(String::from(format!(
                "Expected a declarator, got {}",
                tok
            )))),
        }
    }

    fn parse_const_declarator(&mut self) -> Result<ast::ConstantDeclarator, Error> {
        self.expect(token::Token::Const)?;
        let identifier = self.parse_identifier()?;
        self.expect(token::Token::Assign)?;
        let expression = self.parse_expression()?;

        Ok(ast::ConstantDeclarator {
            identifier,
            expression
        })
    }

    fn parse_type_declarator(&mut self) -> Result<ast::TypeDeclarator, Error> {
        self.expect(token::Token::Type)?;
        let identifier = self.parse_identifier()?;
        self.expect(token::Token::Equals)?;
        let assigned_type = self.parse_type()?;

        Ok(ast::TypeDeclarator {
            identifier,
            type_expression: assigned_type,
        })
    }

    fn parse_variable_declarator(&mut self) -> Result<ast::VariableDeclarator, Error> {
        self.expect(token::Token::Let)?;
        let identifier = self.parse_identifier()?;

        let type_expression = match self.scanner.token().tok {
            token::Token::Colon => {
                self.next();
                Some(self.parse_type()?)
            }
            _ => None,
        };

        let expression = match self.scanner.token().tok {
            token::Token::Assign => {
                self.next();
                Some(self.parse_expression()?)
            }
            _ => None,
        };

        Ok(ast::VariableDeclarator {
            identifier,
            type_expression,
            expression
        })
    }

    fn parse_function_declarator(&mut self) -> Result<ast::FunctionDeclarator, Error> {
        self.expect(token::Token::Function)?;
        let identifier = self.parse_identifier()?;
        let parameters = self.parse_formal_parameter_list()?;
        self.expect(token::Token::Arrow)?;

        let return_type = match self.scanner.token().tok {
            token::Token::LCurlyB => None,
            token::Token::Arrow => {
                self.next();
                Some(self.parse_type()?)
            }
            tok => return Err(Error::new(String::from(format!("Expected either a return type or function start but got {}", tok))))
        };
        self.expect(token::Token::LCurlyB)?;
        let block = self.parse_block();
        self.expect(token::Token::RCurlyB)?;

        Ok(ast::FunctionDeclarator {
            identifier,
            parameters,
            return_type,
            block,
        })
    }

    fn parse_formal_parameter_list(&mut self) -> Result<Vec<ast::Parameter>, Error> {
        let mut params = vec![];
        self.expect(token::Token::LParen)?;

        match self.scanner.token().tok {
            token::Token::RParen => {
                self.next();
                return Ok(params);
            }
            _ => params.push(self.parse_parameter()?),
        }

        loop {
            match self.scanner.token().tok {
                token::Token::RParen => break,
                token::Token::Comma => {
                    self.next();
                    params.push(self.parse_parameter()?);
                }
                tok => return Err(Error::new(format!("Unexpected token {:?}", tok))),
            }
        }
        self.expect(token::Token::RParen)?;

        Ok(params)
    }

    fn parse_parameter(&mut self) -> Result<ast::Parameter, Error> {
        let is_const = match self.scanner.token().tok {
            token::Token::Const => {
                self.next();
                true
            },
            _ => false
        };
        let identifier = self.parse_identifier()?;
        self.expect(token::Token::Colon)?;
        let type_expression = self.parse_type()?;

        Ok(ast::Parameter {
            is_const,
            identifier,
            type_expression,
        })
    }

    fn parse_struct_declarator(&mut self) -> Result<ast::StructDeclarator, Error> {
        self.expect(token::Token::Struct)?;
        let identifier = self.parse_identifier()?;
        let members = self.parse_struct_members()?;

        Ok(ast::StructDeclarator {
            identifier,
            is_soa: false,
            members,
        })
    }

    fn parse_struct_members(&mut self) -> Result<Vec<ast::StructMember>, Error> {
        let mut members = vec![];
        self.expect(token::Token::LCurlyB)?;

        loop {
            match self.scanner.token().tok {
                token::Token::RCurlyB => break,
                _ => {
                    members.push(self.parse_struct_member()?);
                    self.expect(token::Token::Semicolon)?;
                }
            }
        }
        self.expect(token::Token::RCurlyB)?;

        Ok(members)
    }

    fn parse_struct_member(&mut self) -> Result<ast::StructMember, Error> {
        let identifier = self.parse_identifier()?;
        self.expect(token::Token::Colon)?;

        let is_owned = match self.scanner.token().tok {
            token::Token::Owned => {
                self.next();
                true
            },
            _ => false
        };

        let type_expression = self.parse_type()?;

        let has_default_value = self.scanner.token().tok == token::Token::Assign;
        let default_value = if has_default_value {
            self.next();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(ast::StructMember {
            identifier,
            is_owned,
            type_expression,
            default_value,
        })
    }

    fn parse_enum_declarator(&mut self) -> Result<ast::EnumDeclarator, Error> {
        self.expect(token::Token::Enum)?;
        let identifier = self.parse_identifier()?;
        let values = self.parse_enum_value_list()?;

        Ok(ast::EnumDeclarator { identifier, values })
    }

    fn parse_enum_value_list(&mut self) -> Result<Vec<ast::Ident>, Error> {
        let mut values = vec![];
        self.expect(token::Token::LCurlyB)?;

        loop {
            match self.scanner.token().tok {
                token::Token::RCurlyB => break,
                _ => {
                    values.push(self.parse_identifier()?);
                    self.expect(token::Token::Semicolon)?;
                }
            }
        }
        self.expect(token::Token::RCurlyB)?;

        Ok(values)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Error> {
        match self.scanner.token().tok {
            token::Token::Import => Ok(ast::Statement::ImportStatement(self.parse_import()?)),
            token::Token::LCurlyB => Ok(ast::Statement::InnerBlock(self.parse_inner_block()?)),
            token::Token::If => Ok(ast::Statement::IfStatement(self.parse_if()?)),
            token::Token::For => Ok(ast::Statement::LoopStatement(self.parse_loop()?)),
            token::Token::While => Ok(ast::Statement::LoopStatement(self.parse_loop()?)),
            token::Token::Do => Ok(ast::Statement::LoopStatement(self.parse_loop()?)),
            token::Token::Break => Ok(ast::Statement::JumpStatement(self.parse_jump()?)),
            token::Token::Continue => Ok(ast::Statement::JumpStatement(self.parse_jump()?)),
            token::Token::Return => Ok(ast::Statement::JumpStatement(self.parse_jump()?)),
            _ => Ok(ast::Statement::Expression(self.parse_expression()?)),
        }
    }

    fn parse_import(&mut self) -> Result<ast::ImportStatement, Error> {
        self.next();
        Err(Error::new(String::from("Imports not implemented")))
    }

    fn parse_inner_block(&mut self) -> Result<ast::Block, Error> {
        self.next();
        Err(Error::new(String::from("Inner blocks not implemented")))
    }

    fn parse_if(&mut self) -> Result<ast::IfStatement, Error> {
        self.next();
        Err(Error::new(String::from("If statements not implemented")))
    }

    fn parse_loop(&mut self) -> Result<ast::LoopStatement, Error> {
        self.next();
        Err(Error::new(String::from("Loops not implemented")))
    }

    fn parse_jump(&mut self) -> Result<ast::JumpStatement, Error> {
        self.next();
        Err(Error::new(String::from("Jumps not implemented")))
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, Error> {
        self.parse_assignment_expression()
    }

    // Right-associative
    fn parse_assignment_expression(&mut self) -> Result<ast::Expression, Error> {
        let assignment_operators = vec![
            token::Token::Assign,
            token::Token::AddAssign,
            token::Token::SubAssign,
            token::Token::MulAssign,
            token::Token::QuoAssign,
            token::Token::ModAssign,
            token::Token::BitOrAssign,
            token::Token::BitAndAssign,
            token::Token::XorAssign,
            token::Token::ShiftLAssign,
            token::Token::ShiftRAssign,
        ];
        let lhs = self.parse_conditional_expression()?;
        let tok = self.scanner.token().tok;
        if assignment_operators.contains(&tok) {
            let assignment_operator = tok;
            self.next();
            let rhs = self.parse_expression()?;
            Ok(ast::Expression::BinaryExpression(ast::BinaryExpression {
                left_hand_side: Box::new(lhs),
                operator: assignment_operator,
                right_hand_side: Box::new(rhs),
            }))
        } else {
            Ok(lhs)
        }
    }

    // Right-associative
    fn parse_conditional_expression(&mut self) -> Result<ast::Expression, Error> {
        let condition = self.parse_comparing_expression()?;
        let tok = self.scanner.token().tok;
        if tok == token::Token::Question {
            self.next();
            let true_value = Box::new(self.parse_expression()?);
            self.expect(token::Token::Colon)?;
            let false_value = Box::new(self.parse_expression()?);
            Ok(ast::Expression::TernaryExpression(ast::TernaryExpression {
                condition: Box::new(condition),
                true_value,
                false_value,
            }))
        } else {
            Ok(condition)
        }
    }

    // Right-associative
    fn parse_comparing_expression(&mut self) -> Result<ast::Expression, Error> {
        let comparing_operators = vec![
            token::Token::Equals,
            token::Token::GreaterThan,
            token::Token::GreaterThanEquals,
            token::Token::NotEquals,
            token::Token::LessThan,
            token::Token::LessThanEquals,
            token::Token::In,
        ];

        let lhs = self.parse_comparand()?;
        let tok = self.scanner.token().tok;
        if comparing_operators.contains(&tok) {
            let comparing_operator = tok;
            self.next();
            let rhs = self.parse_expression()?;
            Ok(ast::Expression::BinaryExpression(ast::BinaryExpression {
                left_hand_side: Box::new(lhs),
                operator: comparing_operator,
                right_hand_side: Box::new(rhs),
            }))
        } else {
            Ok(lhs)
        }
    }

    // Left-associative
    fn parse_comparand(&mut self) -> Result<ast::Expression, Error> {
        let adding_operators = vec![
            token::Token::Add,
            token::Token::Sub,
            token::Token::Or,
            token::Token::BitOr,
        ];

        let lhs = self.parse_term()?;
        let mut expr = lhs;

        loop {
            let tok = self.scanner.token().tok;
            if adding_operators.contains(&tok) {
                let adding_operator = tok;
                self.next();

                let rhs = self.parse_term()?;
                expr = ast::Expression::BinaryExpression(ast::BinaryExpression {
                    left_hand_side: Box::new(expr),
                    operator: adding_operator,
                    right_hand_side: Box::new(rhs),
                });
            } else {
                break;
            }
        }
        Ok(expr)
    }

    // Left-associative
    fn parse_term(&mut self) -> Result<ast::Expression, Error> {
        let multiplying_operators = vec![
            token::Token::Mul,
            token::Token::Quo,
            token::Token::And,
            token::Token::BitAnd,
        ];

        let lhs = self.parse_factor()?;
        let mut expr = lhs;

        loop {
            let tok = self.scanner.token().tok;
            if multiplying_operators.contains(&tok) {
                let multiplying_operator = tok;
                self.next();
                let rhs = self.parse_factor()?;

                expr = ast::Expression::BinaryExpression(ast::BinaryExpression {
                    left_hand_side: Box::new(expr),
                    operator: multiplying_operator,
                    right_hand_side: Box::new(rhs),
                });
            } else {
                break;
            }
        }
        Ok(expr)
    }

    // Left-associative
    fn parse_factor(&mut self) -> Result<ast::Expression, Error> {
        let shifting_operators = vec![token::Token::ShiftL, token::Token::ShiftR];

        let lhs = self.parse_unary()?;
        let mut expr = lhs;

        loop {
            let tok = self.scanner.token().tok;
            if shifting_operators.contains(&tok) {
                let shifting_operator = tok;
                self.next();

                let rhs = self.parse_unary()?;
                expr = ast::Expression::BinaryExpression(ast::BinaryExpression {
                    left_hand_side: Box::new(expr),
                    operator: shifting_operator,
                    right_hand_side: Box::new(rhs),
                });
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<ast::Expression, Error> {
        let prefix_unary_operators = vec![
            token::Token::Not,
            token::Token::Sub,
            token::Token::Increment,
            token::Token::Decrement,
        ];

        let postfix_unary_operators = vec![token::Token::Increment, token::Token::Decrement];

        let tok = self.scanner.token().tok;
        if prefix_unary_operators.contains(&tok) {
            self.next();
            let rhs = self.parse_unary()?;
            Ok(ast::Expression::UnaryPrefixExpression(
                ast::UnaryExpression {
                    operator: tok,
                    expression: Box::new(rhs),
                },
            ))
        } else {
            let mut expr = self.parse_primary_expression()?;
            loop {
                let tok = self.scanner.token().tok;
                if postfix_unary_operators.contains(&tok) {
                    self.next();
                    expr = ast::Expression::UnaryPostfixExpression(ast::UnaryExpression {
                        operator: tok,
                        expression: Box::new(expr),
                    });
                } else {
                    break;
                }
            }
            Ok(expr)
        }
    }

    fn parse_primary_expression(&mut self) -> Result<ast::Expression, Error> {
        let tok = self.scanner.token().tok;
        match tok {
            token::Token::Integer(_x) => {
                self.next();
                Ok(ast::Expression::PrimaryExpression(
                    ast::PrimaryExpression::Literal(tok),
                ))
            }
            token::Token::String(_x) => {
                self.next();
                Ok(ast::Expression::PrimaryExpression(
                    ast::PrimaryExpression::Literal(tok),
                ))
            }
            token::Token::Null => {
                self.next();
                Ok(ast::Expression::PrimaryExpression(
                    ast::PrimaryExpression::Null,
                ))
            }
            token::Token::LParen => {
                self.next(); // Consume left paren
                             // Determine if this is a lambda or a subexpression
                let x = self.scanner.peek().tok;
                if x == token::Token::Colon {
                    // TODO: Lambdas
                    Err(Error::new(String::from("Lambdas are not yet supported")))
                } else {
                    let r = self.parse_expression()?;
                    self.expect(token::Token::RParen)?;
                    Ok(ast::Expression::PrimaryExpression(
                        ast::PrimaryExpression::SubExpression(Box::new(r)),
                    ))
                }
            }
            _ => Ok(ast::Expression::PrimaryExpression(
                ast::PrimaryExpression::Reference(self.parse_reference()?),
            )),
        }
    }

    fn parse_reference(&mut self) -> Result<ast::Reference, Error> {
        let tok = self.scanner.token().tok;
        let r = match tok {
            token::Token::At => {
                self.next();
                ast::Reference::AddressOf(Box::new(self.parse_reference()?))
            }
            token::Token::Mul => {
                self.next();
                ast::Reference::Dereference(Box::new(self.parse_reference()?))
            }
            _ => ast::Reference::Ident(self.parse_identifier()?),
        };

        // TODO: member access
        // TODO: function call
        // TODO: constructor call
        // TODO: array reference
        // TODO: cast reference

        Ok(r)
    }

    fn parse_type(&mut self) -> Result<ast::TypeExpression, Error> {
        let mut type_expr = self.parse_unary_type()?;
        loop {
            let tok = self.scanner.token().tok;
            match tok {
                token::Token::BitOr => {
                    // Type union
                    self.next();
                    let other_type = self.parse_unary_type()?;
                    type_expr = ast::TypeExpression::TypeUnion(ast::TypeUnion {
                        first_type: Box::new(type_expr),
                        second_type: Box::new(other_type),
                    });
                }
                _ => {
                    break;
                }
            }
        }
        Ok(type_expr)
    }

    fn parse_unary_type(&mut self) -> Result<ast::TypeExpression, Error> {
        let tok = self.scanner.token().tok;
        let mut type_expr = match tok {
            token::Token::Mul => {
                self.next();
                ast::TypeExpression::PointerType(Box::new(self.parse_unary_type()?))
            }
            token::Token::LSquareB => {
                self.next();
                let tok = self.scanner.token().tok;
                match tok {
                    token::Token::DotDot => {
                        // Unsized array
                        self.next();
                        self.expect(token::Token::RSquareB)?;
                        let array_type = self.parse_unary_type()?;
                        ast::TypeExpression::UnsizedArrayType(Box::new(array_type))
                    }
                    _ => {
                        // Sized array
                        let array_size = self.parse_expression()?;
                        self.expect(token::Token::RSquareB)?;
                        let array_type = self.parse_unary_type()?;
                        ast::TypeExpression::SizedArrayType(ast::SizedArrayType {
                            size: array_size,
                            type_expression: Box::new(array_type),
                        })
                    }
                }
            }
            token::Token::Typeof => {
                self.next();
                let expr = self.parse_expression()?;
                ast::TypeExpression::TypeofExpression(expr)
            }
            token::Token::LParen => {
                self.next();
                let t = self.parse_type()?;
                self.expect(token::Token::RParen)?;
                t
            }
            _ => ast::TypeExpression::NamedType(self.parse_identifier()?),
        };
        loop {
            let tok = self.scanner.token().tok;
            match tok {
                token::Token::Question => {
                    // Optional type
                    self.next();
                    type_expr = ast::TypeExpression::OptionalType(Box::new(type_expr));
                }
                _ => {
                    break;
                }
            }
        }
        Ok(type_expr)
    }

    fn parse_identifier(&mut self) -> Result<ast::Ident, Error> {
        match self.scanner.token().tok {
            token::Token::Ident(sym) => {
                self.next();
                Ok(ast::Ident { name: sym })
            }
            tok => Err(Error::new(format!(
                "Expected identifier but got {}",
                tok
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expect() {
        let mut context = Context::new();
        let mut p = Parser::new("test.cat", "; 21", None, &mut context);
        let res1 = p.expect(token::Token::Semicolon);
        let res2 = p.expect(token::Token::Semicolon);
        let res3 = p.expect(token::Token::Semicolon);

        assert!(res1.is_ok());
        assert!(res2.is_err());
        assert!(res3.is_err());
    }

    #[test]
    fn test_is_declaration_starter() {
        let r1 = Parser::is_declaration_starter(token::Token::Function);
        let r2 = Parser::is_declaration_starter(token::Token::Comma);

        assert!(r1);
        assert!(!r2);
    }
}