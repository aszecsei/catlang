use crate::syntax::ast::*;

/// A `SourceUnit` is the top level construct of the grammar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SourceUnit<'ast> {
    pub blocks: NodeList<'ast, Block<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Import<'ast> {
    pub symbol: IdentifierNode<'ast>,
    pub alias: Option<IdentifierNode<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Block<'ast> {
    Declaration(DeclarationNode<'ast>),
    Statement(StatementNode<'ast>),
}

pub type BlockNode<'ast> = Node<'ast, Block<'ast>>;
