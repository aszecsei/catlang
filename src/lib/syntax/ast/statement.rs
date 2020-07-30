use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Statement<'ast> {
    Import(ImportStatement<'ast>),
    InnerBlock(BlockNode<'ast>),
    If(IfStatement<'ast>),
    Loop(LoopStatement<'ast>),
    Jump(JumpStatement<'ast>),
    Expression(ExpressionNode<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImportStatement<'ast> {
    pub is_reexport: bool,
    pub import_list: IdentifierList<'ast>,
    pub path: StringLiteralNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IfStatement<'ast> {
    pub condition: ExpressionNode<'ast>,
    pub true_block: BlockNode<'ast>,
    pub else_block: Option<Conditional<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Conditional<'ast> {
    If(Node<'ast, IfStatement<'ast>>),
    Else(BlockNode<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoopStatement<'ast> {
    ForLoop(ForLoop<'ast>),
    ForInLoop(ForInLoop<'ast>),
    WhileLoop(WhileLoop<'ast>),
    DoWhileLoop(WhileLoop<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ForLoop<'ast> {
    pub initial: Option<ExpressionNode<'ast>>,
    pub condition: Option<ExpressionNode<'ast>>,
    pub update: Option<ExpressionNode<'ast>>,
    pub block: BlockNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ForInLoop<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub range: ExpressionNode<'ast>,
    pub block: BlockNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WhileLoop<'ast> {
    pub condition: ExpressionNode<'ast>,
    pub block: BlockNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JumpStatement<'ast> {
    Break,
    Continue,
    Return(ExpressionNode<'ast>),
}

pub type StatementNode<'ast> = Node<'ast, Statement<'ast>>;
