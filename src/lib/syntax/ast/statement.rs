use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Statement<'ast> {
    InnerBlock(BlockNode<'ast>),
    If(IfStatement<'ast>),
    Loop(LoopStatement<'ast>),
    Jump(JumpStatement<'ast>),
    Expression(ExpressionNode<'ast>),
    Delete(DeleteStatement<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IfStatement<'ast> {
    pub condition: ExpressionNode<'ast>,
    pub true_block: StatementNode<'ast>,
    pub else_block: Option<StatementNode<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoopStatement<'ast> {
    ForLoop(ForLoop<'ast>),
    WhileLoop(WhileLoop<'ast>),
    InfiniteLoop(InfiniteLoop<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ForLoop<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub range: ExpressionNode<'ast>,
    pub statement: StatementNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WhileLoop<'ast> {
    pub is_do_while: bool,
    pub condition: ExpressionNode<'ast>,
    pub statement: StatementNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InfiniteLoop<'ast> {
    pub statement: StatementNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JumpStatement<'ast> {
    Break,
    Continue,
    Return(ExpressionNode<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeleteStatement<'ast> {
    pub deleted: IdentifierNode<'ast>, // TODO: Scoped identifier?
}

pub type StatementNode<'ast> = Node<'ast, Statement<'ast>>;

impl_from! {
    BlockNode => Statement::InnerBlock,
    IfStatement => Statement::If,
    LoopStatement => Statement::Loop,
    JumpStatement => Statement::Jump,
    ExpressionNode => Statement::Expression,
    DeleteStatement => Statement::Delete,

    ForLoop => LoopStatement::ForLoop,
    WhileLoop => LoopStatement::WhileLoop,
    InfiniteLoop => LoopStatement::InfiniteLoop,

    ExpressionNode => JumpStatement::Return,
}
