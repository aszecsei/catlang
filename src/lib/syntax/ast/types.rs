use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeExpression<'ast> {
    PointerType(TypeExpressionNode<'ast>),
    SizedArrayType(SizedArrayType<'ast>),
    UnsizedArrayType(TypeExpressionNode<'ast>),
    TypeUnion(TypeUnionExpression<'ast>),
    TypeofExpression(ExpressionNode<'ast>),
    OptionalType(TypeExpressionNode<'ast>),
    PrimitiveType(PrimitiveType<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeUnionExpression<'ast> {
    pub left: TypeExpressionNode<'ast>,
    pub right: TypeExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SizedArrayType<'ast> {
    pub size: ExpressionNode<'ast>,
    pub inner: TypeExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimitiveType<'ast> {
    S8,
    U8,
    S16,
    U16,
    S32,
    U32,
    S64,
    U64,
    Bool,
    Float,
    Double,
    Null,
    NamedType(IdentifierNode<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnumRepresentationType {
    S8,
    U8,
    S16,
    U16,
    S32,
    U32,
    S64,
    U64,
}

pub type TypeExpressionNode<'ast> = Node<'ast, TypeExpression<'ast>>;
pub type TypeExpressionList<'ast> = NodeList<'ast, TypeExpression<'ast>>;

impl_from! {
    TypeUnionExpression => TypeExpression::TypeUnion,
    SizedArrayType => TypeExpression::SizedArrayType,
    ExpressionNode => TypeExpression::TypeofExpression,
    PrimitiveType => TypeExpression::PrimitiveType,
    IdentifierNode => PrimitiveType::NamedType,
}
