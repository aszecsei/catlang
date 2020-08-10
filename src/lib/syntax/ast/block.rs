use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block<'ast> {
    pub elements: NodeList<'ast, BlockElement<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockElement<'ast> {
    Declaration(Declaration<'ast>),
    Statement(Statement<'ast>),
    Import(Import<'ast>),
}

pub type BlockNode<'ast> = Node<'ast, Block<'ast>>;

impl_from! {
    Declaration => BlockElement::Declaration,
    Statement => BlockElement::Statement,
    Import => BlockElement::Import,
}
