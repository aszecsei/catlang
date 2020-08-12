use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block<'ast> {
    pub elements: NodeList<'ast, BlockElement<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockElement<'ast> {
    Declaration(DeclarationNode<'ast>),
    Statement(StatementNode<'ast>),
    Import(ImportNode<'ast>),
}

pub type BlockNode<'ast> = Node<'ast, Block<'ast>>;
pub type BlockElementNode<'ast> = Node<'ast, BlockElement<'ast>>;

impl_from! {
    DeclarationNode => BlockElement::Declaration,
    StatementNode => BlockElement::Statement,
    ImportNode => BlockElement::Import,
}
