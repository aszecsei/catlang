use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeExpression<'ast> {
    Binary(BinaryTypeExpression<'ast>),
    Unary(UnaryTypeExpression<'ast>),
    Simple(SimpleTypeExpression<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryTypeOperator {
    TypeUnion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryTypeOperator {
    PointerTo,
    SizedArray,
    UnsizedArray,
    Const,
    Volatile,
    Optional,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BinaryTypeExpression<'ast> {
    pub left: TypeExpressionNode<'ast>,
    pub op: BinaryTypeOperator,
    pub right: TypeExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnaryTypeExpression<'ast> {
    pub op: UnaryTypeOperator,
    pub inner: TypeExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimpleTypeExpression<'ast> {
    Typeof(ExpressionNode<'ast>),
    NamedType(NamedType<'ast>),
    PrimitiveType(PrimitiveType),
    SubExpression(TypeExpressionNode<'ast>),
    Any,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NamedType<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub generic_parameters: TypeExpressionList<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    S8,
    U8,
    S16,
    U16,
    S32,
    U32,
    S64,
    U64,
    Char,
    Short,
    Int,
    Long,
    CShort,
    #[allow(clippy::upper_case_acronyms)]
    CUShort,
    CInt,
    #[allow(clippy::upper_case_acronyms)]
    CUInt,
    CLong,
    #[allow(clippy::upper_case_acronyms)]
    CULong,
    CLongLong,
    #[allow(clippy::upper_case_acronyms)]
    CULongLong,
    CLongDouble,
    Bool,
    F32,
    F64,
    Float,
    Double,
    Null,
    NoReturn,
    CVoid,
}

pub type TypeExpressionNode<'ast> = Node<'ast, TypeExpression<'ast>>;
pub type TypeExpressionList<'ast> = NodeList<'ast, TypeExpression<'ast>>;

impl_from! {
    BinaryTypeExpression => TypeExpression::Binary,
    UnaryTypeExpression => TypeExpression::Unary,
    SimpleTypeExpression => TypeExpression::Simple,

    ExpressionNode => SimpleTypeExpression::Typeof,
    NamedType => SimpleTypeExpression::NamedType,
    TypeExpressionNode => SimpleTypeExpression::SubExpression,
}

impl<'ast> From<PrimitiveType> for SimpleTypeExpression<'ast> {
    fn from(pt: PrimitiveType) -> Self {
        SimpleTypeExpression::PrimitiveType(pt)
    }
}
