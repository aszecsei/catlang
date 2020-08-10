use toolshed::list::GrowableList;

use crate::syntax::ast::*;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn source_unit(&mut self) -> SourceUnitNode<'ast> {
        let start = self.current_span.start as u32;

        let mut declarations = vec![];
        let mut imports = vec![];
        let mut exports = vec![];
        let mut is_script = true;
        while self.current_token != Token::EndOfFile {
            match self.current_token {
                Token::Export => {
                    is_script = false;
                    if let Ok(export) = self.export() {
                        exports.push(export);
                    }
                }
                Token::Import => {
                    is_script = false;
                    if let Ok(import) = self.import() {
                        imports.push(import);
                    }
                }
                _ => {
                    if let Ok(declaration) = self.declaration_node() {
                        declarations.push(declaration);
                    }
                }
            }
        }

        let end = self.last_span.end as u32;

        if is_script {
            let elements = GrowableList::new();
            for declaration in declarations {
                elements.push(
                    self.arena,
                    self.node_at(declaration.start, declaration.end, declaration),
                );
            }
            self.node_at(
                start,
                end,
                Script {
                    elements: elements.as_list(),
                },
            )
        } else {
            let elements = GrowableList::new();
            for import in imports {
                elements.push(self.arena, self.node_at(import.start, import.end, import));
            }
            for declaration in declarations {
                elements.push(
                    self.arena,
                    self.node_at(declaration.start, declaration.end, declaration),
                );
            }
            for export in exports {
                elements.push(self.arena, self.node_at(export.start, export.end, export));
            }
            self.node_at(
                start,
                end,
                Module {
                    elements: elements.as_list(),
                },
            )
        }
    }
}
