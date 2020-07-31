use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expression<'ast> {
    TernaryExpression(TernaryExpression<'ast>),
    BinaryExpression(BinaryExpression<'ast>),
    PostfixExpression(PostfixExpression<'ast>),
    PrefixExpression(PrefixExpression<'ast>),
    AssignmentExpression(AssignmentExpression<'ast>),
    TupleExpression(TupleExpression<'ast>),
    PrimitiveExpression(Primitive<'ast>),
    CallExpression(CallExpression<'ast>),
    ConstructorCallExpression(ConstructorCallExpression<'ast>),
    IndexExpression(IndexExpression<'ast>),
    CastExpression(CastExpression<'ast>),
    MemberAccessExpression(MemberAccessExpression<'ast>),
    IdentifierExpression(Identifier<'ast>),
    LambdaExpression(LambdaExpression<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Primitive<'ast> {
    Null,
    Bool(bool),
    DecimalNumber(&'ast str),
    DecimalFloat(&'ast str),
    HexadecimalNumber(&'ast str),
    OctalNumber(&'ast str),
    BinaryNumber(&'ast str),
    String(&'ast str),
    Char(&'ast str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    In,
    As,
    Addition,
    Subtraction,
    BitOr,
    BitXor,
    LogicalOr,
    Multiplication,
    Division,
    Remainder,
    BitAnd,
    LogicalAnd,
    BitShiftLeft,
    BitShiftRight,
    NullCoalesce,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssignmentOperator {
    Plain,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    BitShiftLeft,
    BitShiftRight,
    BitAnd,
    BitXor,
    BitOr,
    LogicalAnd,
    LogicalOr,
    NullCoalesce,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrefixOperator {
    LogicalNot,
    BitNot,
    Delete,
    Increment,
    Decrement,
    Plus,
    Minus,
    AddressOf,
    PointerDeref,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PostfixOperator {
    Increment,
    Decrement,
    NullCoersion,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TernaryExpression<'ast> {
    pub condition: ExpressionNode<'ast>,
    pub when_true: ExpressionNode<'ast>,
    pub when_false: ExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BinaryExpression<'ast> {
    pub left: ExpressionNode<'ast>,
    pub operator: Node<'ast, BinaryOperator>,
    pub right: ExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PostfixExpression<'ast> {
    pub operand: ExpressionNode<'ast>,
    pub operator: Node<'ast, PostfixOperator>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrefixExpression<'ast> {
    pub operator: Node<'ast, PrefixOperator>,
    pub operand: ExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TupleExpression<'ast> {
    pub expressions: ExpressionList<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignmentExpression<'ast> {
    pub left: ExpressionNode<'ast>,
    pub operator: Node<'ast, AssignmentOperator>,
    pub right: ExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallExpression<'ast> {
    pub callee: ExpressionNode<'ast>,
    pub arguments: ExpressionList<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstructorCallExpression<'ast> {
    pub callee: TypeExpressionNode<'ast>,
    pub arguments: ExpressionList<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndexExpression<'ast> {
    pub array: ExpressionNode<'ast>,
    pub index: ExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastExpression<'ast> {
    pub left: ExpressionNode<'ast>,
    pub forced: bool,
    pub cast_to: TypeExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MemberAccessExpression<'ast> {
    pub object: ExpressionNode<'ast>,
    pub null_condition: bool,
    pub member: IdentifierNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LambdaExpression<'ast> {
    pub parameters: NodeList<'ast, Parameter<'ast>>,
    pub block: BlockNode<'ast>,
}

pub type ExpressionNode<'ast> = Node<'ast, Expression<'ast>>;
pub type ExpressionList<'ast> = NodeList<'ast, Expression<'ast>>;

impl_from! {
    TernaryExpression => Expression::TernaryExpression,
    BinaryExpression => Expression::BinaryExpression,
    PostfixExpression => Expression::PostfixExpression,
    PrefixExpression => Expression::PrefixExpression,
    AssignmentExpression => Expression::AssignmentExpression,
    TupleExpression => Expression::TupleExpression,
    Primitive => Expression::PrimitiveExpression,
    CallExpression => Expression::CallExpression,
    ConstructorCallExpression => Expression::ConstructorCallExpression,
    IndexExpression => Expression::IndexExpression,
    CastExpression => Expression::CastExpression,
    MemberAccessExpression => Expression::MemberAccessExpression,
    Identifier => Expression::IdentifierExpression,
}
