use super::*;

impl<'ast> Parser<'ast> {
    pub fn import(&mut self) -> Result<ImportNode<'ast>> {
        let start = self.start_then_advance();
        let import_list: ImportList = self.import_list()?;

        self.expect(Token::From);
        let path = self.string_literal_node()?;
        self.eat(Token::Semicolon);

        Ok(self.node_at(start, path.end, Import { import_list, path }))
    }

    fn import_list(&mut self) -> Result<ImportList<'ast>> {
        if self.eat(Token::Mul) {
            // Glob import
            self.expect(Token::As);
            let identifier = self.identifier_node()?;
            Ok(GlobImportList { identifier }.into())
        } else {
            // Named import
            self.expect(Token::LCurlyB);
            let names = GrowableList::new();
            loop {
                match self.current_token {
                    Token::RCurlyB => break,
                    _ => {
                        let identifier = self.identifier_node()?;
                        let renamed_to = if self.eat(Token::As) {
                            Some(self.identifier_node()?)
                        } else {
                            None
                        };
                        let start = identifier.start;
                        let end = if let Some(renamed) = renamed_to {
                            renamed.end
                        } else {
                            identifier.end
                        };
                        names.push(
                            self.arena,
                            self.node_at(
                                start,
                                end,
                                ImportIdentifier {
                                    identifier,
                                    renamed_to,
                                },
                            ),
                        );
                        self.expect_one_of(&[Token::Comma], &[Token::RCurlyB]);
                    }
                }
            }
            self.expect(Token::RCurlyB);
            Ok(NamedImportList {
                imports: names.as_list(),
            }
            .into())
        }
    }

    pub fn export(&mut self) -> Result<ExportNode<'ast>> {
        let start = self.start_then_advance();
        if is_declaration_starter(self.current_token) {
            // Export declaration
            let declaration = self.declaration_node()?;
            Ok(self.node_at(start, declaration.end, declaration))
        } else if self.current_token == Token::LCurlyB || self.current_token == Token::Mul {
            // Re-export
            let exports = self.import_list()?;
            self.expect(Token::From);
            let path = self.string_literal_node()?;
            self.eat(Token::Semicolon);
            Ok(self.node_at(start, path.end, ExportReExport { exports, path }))
        } else {
            // Export statement
            let identifier = self.identifier_node()?;
            let renamed_to = if self.eat(Token::As) {
                Some(self.identifier_node()?)
            } else {
                None
            };
            let end = if let Some(renamed) = renamed_to {
                renamed.end
            } else {
                identifier.end
            };
            self.eat(Token::Semicolon);
            Ok(self.node_at(
                start,
                end,
                ExportStatement {
                    identifier,
                    renamed_to,
                },
            ))
        }
    }
}

pub fn is_declaration_starter(t: Token) -> bool {
    matches!(
        t,
        Token::Const | Token::Type | Token::Let | Token::Function | Token::Struct | Token::Enum
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use toolshed::Arena;

    #[test]
    fn test_glob_import() {
        let source = "import * as vectors from \"./vectors\"";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.import().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_named_imports() {
        let source = "import { Vector2 as Vec2, Vector3 } from \"vectors\";";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.import().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_glob_re_export() {
        let source = "export * as vectors from \"vectors\";";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.export().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_named_re_export() {
        let source = "export { Vector2 as Vec2, Vector3 } from \"vectors\";";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.export().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_export_statement() {
        let source = "export PI;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.export().unwrap();

        assert_debug_snapshot!(res);
    }

    #[test]
    fn test_renamed_export_statement() {
        let source = "export PI_CONST as PI;";
        let arena = Arena::new();
        let mut p = Parser::new(source, &arena);
        let res = p.export().unwrap();

        assert_debug_snapshot!(res);
    }
}
