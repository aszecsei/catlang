use crate::syntax::ast::*;

/// A `SourceUnit` is the top level construct of the grammar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SourceUnit<'ast> {
    pub block: BlockNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block<'ast> {
    pub elements: NodeList<'ast, BlockElement<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockElement<'ast> {
    Declaration(DeclarationNode<'ast>),
    Statement(StatementNode<'ast>),
}

pub type BlockNode<'ast> = Node<'ast, Block<'ast>>;
