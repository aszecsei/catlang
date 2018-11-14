use fnv::FnvHashMap;
use lexer::symbol;
use lexer::token;
use std::rc::Rc;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub elements: Vec<Box<BlockElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockElement {
    Declaration(Declaration),
    Statement(Statement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Declaration {
    pub is_exported: bool,
    pub declarator: Declarator,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Declarator {
    ConstantDeclarator(ConstantDeclarator),
    TypeDeclarator(TypeDeclarator),
    VariableDeclarator(VariableDeclarator),
    FunctionDeclarator(FunctionDeclarator),
    StructDeclarator(StructDeclarator),
    EnumDeclarator(EnumDeclarator),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantDeclarator {
    pub identifier: Ident,
    pub expression: Expression,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeDeclarator {
    pub identifier: Ident,
    pub type_expression: TypeExpression,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub identifier: Ident,
    pub type_expression: Option<TypeExpression>,
    pub expression: Option<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDeclarator {
    pub identifier: Ident,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub is_const: bool,
    pub identifier: Ident,
    pub type_expression: TypeExpression,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructDeclarator {
    pub identifier: Ident,
    pub is_soa: bool,
    pub members: Vec<StructMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructMember {
    pub identifier: Ident,
    pub is_owned: bool,
    pub type_expression: TypeExpression,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumDeclarator {
    pub identifier: Ident,
    pub values: Vec<Ident>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
    ImportStatement(ImportStatement),
    InnerBlock(Block),
    IfStatement(IfStatement),
    LoopStatement(LoopStatement),
    JumpStatement(JumpStatement),
    Expression(Expression),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportStatement {
    pub is_reexport: bool,
    pub import_list: Vec<ImportIdentifier>,
    pub path: symbol::Symbol,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportIdentifier {
    pub old_name: Ident,
    pub new_name: Ident,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub true_block: Block,
    pub else_block: Option<Box<Conditional>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Conditional {
    If(IfStatement),
    Else(Block),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LoopStatement {
    ForLoop(ForLoop),
    ForInLoop(ForInLoop),
    WhileLoop(WhileLoop),
    DoWhileLoop(WhileLoop),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForLoop {
    pub initial_expression: Option<Expression>,
    pub condition_expression: Option<Expression>,
    pub update_expression: Option<Expression>,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForInLoop {
    pub identifier: Ident,
    pub range: Expression,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhileLoop {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JumpStatement {
    Break,
    Continue,
    Return(Expression),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeExpression {
    PointerType(Box<TypeExpression>),
    SizedArrayType(SizedArrayType),
    UnsizedArrayType(Box<TypeExpression>),
    TypeUnion(TypeUnion),
    TypeofExpression(Expression),
    OptionalType(Box<TypeExpression>),
    NamedType(Ident),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SizedArrayType {
    pub size: Expression,
    pub type_expression: Box<TypeExpression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeUnion {
    pub first_type: Box<TypeExpression>,
    pub second_type: Box<TypeExpression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expression {
    UnaryPrefixExpression(UnaryExpression),
    UnaryPostfixExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    TernaryExpression(TernaryExpression),
    PrimaryExpression(PrimaryExpression),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: token::Token,
    pub expression: Box<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left_hand_side: Box<Expression>,
    pub operator: token::Token,
    pub right_hand_side: Box<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TernaryExpression {
    pub condition: Box<Expression>,
    pub true_value: Box<Expression>,
    pub false_value: Box<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrimaryExpression {
    Literal(token::Token),
    Null,
    SubExpression(Box<Expression>),
    LambdaExpression(LambdaExpression),
    Reference(Reference),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LambdaExpression {
    pub parameters: Vec<Parameter>,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Reference {
    Ident(Ident),
    AddressOf(Box<Reference>),
    Dereference(Box<Reference>),
    MemberAccess(MemberAccess),
    FunctionCall(FunctionCall),
    ConstructorCall(ConstructorCall),
    ArrayAccess(ArrayAccess),
    CastReference(CastReference),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberAccess {
    pub reference: Box<Reference>,
    pub member_ident: Ident,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub function: Box<Reference>,
    pub args: Vec<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstructorCall {
    pub is_heap: bool,
    pub object_type: Box<Reference>,
    pub constructor_args: Option<Vec<Expression>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayAccess {
    pub arr_reference: Box<Reference>,
    pub indices: Vec<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CastReference {
    pub is_unsafe: bool,
    pub reference: Box<Reference>,
    pub new_type: Box<TypeExpression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ident {
    pub name: symbol::Symbol,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Object {
    pub name: symbol::Symbol,
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub parent: Option<Rc<Scope>>,
    table: FnvHashMap<symbol::Symbol, Object>,
}

impl Scope {
    pub fn new(parent: Option<Rc<Scope>>) -> Scope {
        Scope {
            parent,
            table: FnvHashMap::default(),
        }
    }

    pub fn insert(&mut self, ob: Object) -> Option<Object> {
        self.table.insert(ob.name, ob)
    }

    pub fn lookup(&self, ident: &symbol::Symbol) -> Option<&Object> {
        self.table.get(ident)
    }
}
