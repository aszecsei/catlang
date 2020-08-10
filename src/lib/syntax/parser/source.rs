use toolshed::list::GrowableList;

use crate::syntax::ast::*;
use crate::syntax::error::Result;
use crate::syntax::lexer::Token;
use crate::syntax::parser::Parser;

impl<'ast> Parser<'ast> {
    pub fn source_unit(&mut self) -> SourceUnitNode<'ast> {
        let start = self.current_span.start as u32;

        let elements = GrowableList::new();
        let mut is_script = true;
        loop {
            match self.current_token {
                Token::EndOfFile => break,
                Token::Semicolon => self.bump(),
                Token::Export => {
                    // TODO: =BUG= If this is an export declaration, it might have attributes before it.
                    is_script = false;
                    let res = self.module_element_node();
                    if let Ok(element_node) = res {
                        elements.push(self.arena, element_node);
                    }
                }
                Token::Import => {
                    is_script = false;
                    let res = self.module_element_node();
                    if let Ok(element_node) = res {
                        elements.push(self.arena, element_node);
                    }
                }
                _ => {
                    let res = self.module_element_node();
                    if let Ok(element_node) = res {
                        elements.push(self.arena, element_node);
                    }
                }
            }
        }
        let end = self.last_span.end as u32;

        self.node_at(
            start,
            end,
            Module {
                elements: elements.as_list(),
                is_script,
            },
        )
    }

    fn module_element_node(&mut self) -> Result<Node<'ast, ModuleElement<'ast>>> {
        let (start, end, element): (u32, u32, ModuleElement) = match self.current_token {
            Token::Export => {
                let element = self.export()?;
                (element.start, element.end, element.into())
            }
            Token::Import => {
                let element = self.import()?;
                (element.start, element.end, element.into())
            }
            _ => {
                let element = self.declaration_node()?;
                (element.start, element.end, element.into())
            }
        };

        Ok(self.node_at(start, end, element))
    }
}
