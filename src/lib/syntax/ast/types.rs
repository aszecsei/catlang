#[macro_use]
use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeExpression<'ast> {
    PointerType(TypeExpressionNode<'ast>),
    SizedArrayType(TypeExpressionNode<'ast>),
    UnsizedArrayType(TypeExpressionNode<'ast>),
    TypeUnion(TypeUnionExpression<'ast>),
    TypeofExpression(ExpressionNode<'ast>),
    OptionalType(TypeExpressionNode<'ast>),
    NamedType(&'ast str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeUnionExpression<'ast> {
    pub left: TypeExpressionNode<'ast>,
    pub right: TypeExpressionNode<'ast>,
}

pub type TypeExpressionNode<'ast> = Node<'ast, TypeExpression<'ast>>;
pub type TypeExpressionList<'ast> = NodeList<'ast, TypeExpression<'ast>>;

impl_from! {
    TypeUnionExpression => TypeExpression::TypeUnion,
}
