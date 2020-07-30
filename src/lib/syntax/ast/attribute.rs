use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attribute<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub parameters: ExpressionList<'ast>,
}

pub type AttributeNode<'ast> = Node<'ast, Attribute<'ast>>;
pub type AttributeList<'ast> = NodeList<'ast, Attribute<'ast>>;
