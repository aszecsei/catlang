use super::*;
use crate::syntax::lexer::Token;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expression<'ast> {
    TernaryExpression(TernaryExpression<'ast>),
    BinaryExpression(BinaryExpression<'ast>),
    PostfixExpression(PostfixExpression<'ast>),
    PrefixExpression(PrefixExpression<'ast>),
    AssignmentExpression(AssignmentExpression<'ast>),
    PrimitiveExpression(Primitive<'ast>),
    CallExpression(CallExpression<'ast>),
    ConstructorCallExpression(ConstructorCallExpression<'ast>),
    IndexExpression(IndexExpression<'ast>),
    CastExpression(CastExpression<'ast>),
    MemberAccessExpression(MemberAccessExpression<'ast>),
    IdentifierExpression(IdentifierNode<'ast>),
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
    Cast,
    ForcedCast,
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
    RangeExclusive,
    RangeInclusive,
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
    Increment,
    Decrement,
    Plus,
    Minus,
}

impl From<Token> for PrefixOperator {
    fn from(t: Token) -> Self {
        match t {
            Token::Not => PrefixOperator::LogicalNot,
            Token::BitNot => PrefixOperator::BitNot,
            Token::Increment => PrefixOperator::Increment,
            Token::Decrement => PrefixOperator::Decrement,
            Token::Add => PrefixOperator::Plus,
            Token::Sub => PrefixOperator::Minus,
            t => panic!("invalid prefix operator {:?}", t),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PostfixOperator {
    Increment,
    Decrement,
    NullConditional,
    NullConditionalIndex,
    NullForgiving,
}

impl From<Token> for PostfixOperator {
    fn from(t: Token) -> Self {
        match t {
            Token::Increment => PostfixOperator::Increment,
            Token::Decrement => PostfixOperator::Decrement,
            Token::Not => PostfixOperator::NullForgiving,
            Token::Question => PostfixOperator::NullConditional,
            t => panic!("invalid postfix operator {:?}", t),
        }
    }
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
    pub operator: BinaryOperator,
    pub right: ExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PostfixExpression<'ast> {
    pub operand: ExpressionNode<'ast>,
    pub operator: PostfixOperator,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrefixExpression<'ast> {
    pub operator: PrefixOperator,
    pub operand: ExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssignmentExpression<'ast> {
    pub left: ExpressionNode<'ast>,
    pub operator: AssignmentOperator,
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
    Primitive => Expression::PrimitiveExpression,
    CallExpression => Expression::CallExpression,
    ConstructorCallExpression => Expression::ConstructorCallExpression,
    IndexExpression => Expression::IndexExpression,
    CastExpression => Expression::CastExpression,
    MemberAccessExpression => Expression::MemberAccessExpression,
    IdentifierNode => Expression::IdentifierExpression,
}
