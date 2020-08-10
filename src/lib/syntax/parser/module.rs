use super::*;

impl<'ast> Parser<'ast> {
    pub fn import(&mut self) -> Result<ImportNode<'ast>> {
        let start = self.start_then_advance();
        let import_list: ImportList = if self.eat(Token::Mul) {
            // Glob import
            self.expect(Token::As);
            let identifier = self.identifier_node()?;
            GlobImportList { identifier }.into()
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
            NamedImportList {
                imports: names.as_list(),
            }
            .into()
        };

        self.expect(Token::From);
        let path = self.string_literal_node()?;
        self.eat(Token::Semicolon);

        Ok(self.node_at(start, path.end, Import { import_list, path }))
    }

    pub fn export(&mut self) -> Result<ExportNode<'ast>> {
        Err(Error::NotImplementedError)
    }
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
}
