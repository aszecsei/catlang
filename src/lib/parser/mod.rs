pub mod ast;

use lexer;
use lexer::token;
use std::fmt;
use std::rc::Rc;

use log::error;

pub struct Error {
    // TODO: Add position information
    msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error {
    pub fn new(msg: String) -> Error {
        Error { msg }
    }
}

pub struct Parser<'a> {
    fname: &'a str,
    errors: Vec<Error>,
    scanner: lexer::Scanner<'a>,
    current_scope: Rc<ast::Scope>,
    top_scope: Rc<ast::Scope>,
}

impl<'a> Parser<'a> {
    pub fn parse_file(fname: &'a str, src: &'a str) -> ast::Block {
        let mut p = Parser::new(fname, src, None);
        let res = p.parse_block();
        // TODO: Kinder error logging (i.e. hide > 10 errors)
        for e in p.errors {
            error!("{}", e.msg);
        }
        res
    }

    fn expect(&mut self, tok: token::Token) -> Result<(), Error> {
        let r = match self.scanner.current_lexeme {
            Some(lexeme) => {
                if lexeme.token != tok {
                    Err(Error::new(format!(
                        "Expected '{}' but got '{}'",
                        tok, lexeme.token
                    )))
                } else {
                    Ok(())
                }
            }
            None => Err(Error::new(format!(
                "Expected '{}' but no lexeme was found!",
                tok
            ))),
        };
        self.next();
        r
    }

    fn new(fname: &'a str, src: &'a str, scope: Option<ast::Scope>) -> Self {
        let scope = match scope {
            Some(s) => s,
            None => ast::Scope::new(None),
        };
        let rc_scope = Rc::new(scope);
        Parser {
            fname,
            errors: vec![],
            scanner: lexer::Scanner::new(fname, src),
            current_scope: rc_scope.clone(),
            top_scope: rc_scope,
        }
    }

    fn next(&mut self) {
        self.scanner.advance();
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
        loop {
            match self.scanner.current_lexeme {
                Some(l) => {
                    if l.token == token::Token::EOF || l.token == token::Token::RCurlyB {
                        break;
                    }

                    if Parser::is_declaration_starter(l.token) {
                        let declaration = self.parse_declaration();
                        match declaration {
                            Err(e) => self.errors.push(e),
                            Ok(d) => elements.push(Box::new(ast::BlockElement::Declaration(d))),
                        };
                    } else {
                        let statement = self.parse_statement();
                        match statement {
                            Err(e) => self.errors.push(e),
                            Ok(s) => elements.push(Box::new(ast::BlockElement::Statement(s))),
                        };
                    };

                    match self.expect(token::Token::Semicolon) {
                        Err(e) => self.errors.push(e),
                        _ => (),
                    };
                }
                _ => {
                    break;
                }
            }
        }
        ast::Block { elements }
    }

    fn parse_declaration(&mut self) -> Result<ast::Declaration, Error> {
        match self.scanner.current_lexeme {
            Some(l) => {
                let is_exported = if l.token == token::Token::Export {
                    self.next();
                    true
                } else {
                    false
                };
                let declarator = self.parse_declarator()?;
                Ok(ast::Declaration {
                    is_exported,
                    declarator,
                })
            }
            _ => Err(Error::new(String::from("Unexpected end of file"))),
        }
    }

    fn parse_declarator(&mut self) -> Result<ast::Declarator, Error> {
        match self.scanner.current_lexeme {
            Some(l) => match l.token {
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
                _ => Err(Error::new(String::from(format!(
                    "Expected a declarator, got {}",
                    l.token
                )))),
            },
            _ => Err(Error::new(String::from("Unexpected end of file"))),
        }
    }

    fn parse_const_declarator(&mut self) -> Result<ast::ConstantDeclarator, Error> {
        self.expect(token::Token::Const)?;
        let identifier = self.parse_identifier()?;
        self.expect(token::Token::Assign)?;
        let expression = self.parse_expression()?;

        Ok(ast::ConstantDeclarator {
            identifier,
            expression,
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

        let type_expression = match self.scanner.current_lexeme {
            Some(l) => match l.token {
                token::Token::Colon => {
                    self.next();
                    Some(self.parse_type()?)
                }
                _ => None,
            },
            _ => None,
        };

        let expression = match self.scanner.current_lexeme {
            Some(l) => match l.token {
                token::Token::Assign => {
                    self.next();
                    Some(self.parse_expression()?)
                }
                _ => None,
            },
            _ => None,
        };

        Ok(ast::VariableDeclarator {
            identifier,
            type_expression,
            expression,
        })
    }

    fn parse_function_declarator(&mut self) -> Result<ast::FunctionDeclarator, Error> {
        self.expect(token::Token::Function)?;
        let identifier = self.parse_identifier()?;
        let parameters = self.parse_formal_parameter_list()?;
        self.expect(token::Token::Arrow)?;

        let return_type = match self.scanner.current_lexeme {
            Some(l) => match l.token {
                token::Token::LCurlyB => None,
                _ => Some(self.parse_type()?),
            },
            _ => None,
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

        loop {
            match self.scanner.current_lexeme {
                Some(l) => match l.token {
                    token::Token::RParen => break,
                    _ => {
                        params.push(self.parse_parameter()?);
                        self.expect(token::Token::Comma)?;
                    }
                },
                _ => return Err(Error::new(String::from("Unexpected end of file"))),
            }
        }
        self.expect(token::Token::RParen)?;

        Ok(params)
    }

    fn parse_parameter(&mut self) -> Result<ast::Parameter, Error> {
        let identifier = self.parse_identifier()?;
        self.expect(token::Token::Colon)?;
        let type_expression = self.parse_type()?;

        Ok(ast::Parameter {
            is_const: false, // TODO: Handle const params
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
            match self.scanner.current_lexeme {
                Some(l) => match l.token {
                    token::Token::RCurlyB => break,
                    _ => {
                        members.push(self.parse_struct_member()?);
                        self.expect(token::Token::Semicolon)?;
                    }
                },
                _ => return Err(Error::new(String::from("Unexpected end of file"))),
            }
        }
        self.expect(token::Token::RCurlyB)?;

        Ok(members)
    }

    fn parse_struct_member(&mut self) -> Result<ast::StructMember, Error> {
        let identifier = self.parse_identifier()?;
        self.expect(token::Token::Colon)?;

        let is_owned = self.scanner.current_lexeme.unwrap().token == token::Token::Owned; // TODO: Unwrap
        if is_owned {
            self.next();
        }

        let type_expression = self.parse_type()?;

        let has_default_value = self.scanner.current_lexeme.unwrap().token == token::Token::Assign; // TODO: Unwrap
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
            match self.scanner.current_lexeme {
                Some(l) => match l.token {
                    token::Token::RCurlyB => break,
                    _ => {
                        values.push(self.parse_identifier()?);
                        self.expect(token::Token::Semicolon)?;
                    }
                },
                _ => return Err(Error::new(String::from("Unexpected end of file"))),
            }
        }
        self.expect(token::Token::RCurlyB)?;

        Ok(values)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, Error> {
        match self.scanner.current_lexeme {
            Some(l) => match l.token {
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
            },
            _ => Err(Error::new(String::from("Unexpected end of file"))),
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
        if let Some(l) = self.scanner.current_lexeme {
            if assignment_operators.contains(&l.token) {
                let assignment_operator = l.token;
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
        } else {
            Ok(lhs)
        }
    }

    // Right-associative
    fn parse_conditional_expression(&mut self) -> Result<ast::Expression, Error> {
        let condition = self.parse_comparing_expression()?;
        if let Some(l) = self.scanner.current_lexeme {
            if l.token == token::Token::Question {
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
        if let Some(l) = self.scanner.current_lexeme {
            if comparing_operators.contains(&l.token) {
                let comparing_operator = l.token;
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
            if let Some(l) = self.scanner.current_lexeme {
                if adding_operators.contains(&l.token) {
                    let adding_operator = l.token;
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
            if let Some(l) = self.scanner.current_lexeme {
                if multiplying_operators.contains(&l.token) {
                    let multiplying_operator = l.token;
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
            if let Some(l) = self.scanner.current_lexeme {
                if shifting_operators.contains(&l.token) {
                    let shifting_operator = l.token;
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
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<ast::Expression, Error> {
        let unary_operators = vec![
            token::Token::Not,
            token::Token::Sub,
            token::Token::Increment,
            token::Token::Decrement,
        ];

        if let Some(l) = self.scanner.current_lexeme {
            if unary_operators.contains(&l.token) {
                self.next();
                let rhs = self.parse_unary()?;
                Ok(ast::Expression::UnaryPrefixExpression(
                    ast::UnaryExpression {
                        operator: l.token,
                        expression: Box::new(rhs),
                    },
                ))
            } else {
                let mut expr = self.parse_primary_expression()?;
                loop {
                    if let Some(l2) = self.scanner.current_lexeme {
                        if unary_operators.contains(&l2.token) {
                            self.next();
                            expr = ast::Expression::UnaryPostfixExpression(ast::UnaryExpression {
                                operator: l2.token,
                                expression: Box::new(expr),
                            });
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Ok(expr)
            }
        } else {
            Err(Error::new(String::from("Unexpected end of file")))
        }
    }

    fn parse_primary_expression(&mut self) -> Result<ast::Expression, Error> {
        Ok(ast::Expression::PrimaryExpression(
            ast::PrimaryExpression::Reference(ast::Reference::Ident(self.parse_identifier()?)),
        ))
    }

    fn parse_type(&mut self) -> Result<ast::TypeExpression, Error> {
        self.next();
        Err(Error::new(String::from("Type expressions not implemented")))
    }

    fn parse_identifier(&mut self) -> Result<ast::Ident, Error> {
        match self.scanner.current_lexeme {
            Some(l) => match l.token {
                token::Token::Ident(sym) => {
                    self.next();
                    Ok(ast::Ident { name: sym })
                }
                _ => Err(Error::new(format!(
                    "Expected identifier but got {}",
                    l.token
                ))),
            },
            _ => Err(Error::new(String::from("Unexpected end of file"))),
        }
    }
}
