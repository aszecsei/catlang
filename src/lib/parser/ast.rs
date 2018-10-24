use lexer::symbol;
use lexer::token;
use std::rc::Rc;

pub struct Block {
    pub elements: Vec<Box<BlockElement>>,
}

pub enum BlockElement {
    Declaration(Declaration),
    Statement(Statement),
}

pub struct Declaration {
    pub is_exported: bool,
    pub declarator: Declarator,
}

pub enum Declarator {
    ConstantDeclarator(ConstantDeclarator),
    TypeDeclarator(TypeDeclarator),
    VariableDeclarator(VariableDeclarator),
    FunctionDeclarator(FunctionDeclarator),
    StructDeclarator(StructDeclarator),
    EnumDeclarator(EnumDeclarator),
}

pub struct ConstantDeclarator {
    pub identifier: Ident,
    pub expression: Expression,
}

pub struct TypeDeclarator {
    pub identifier: Ident,
    pub type_expression: TypeExpression,
}

pub struct VariableDeclarator {
    pub identifier: Ident,
    pub type_expression: Option<TypeExpression>,
    pub expression: Option<Expression>,
}

pub struct FunctionDeclarator {
    pub identifier: Ident,
    pub parameters: Vec<Parameter>,
    pub block: Block,
}

pub struct Parameter {
    pub is_const: bool,
    pub identifier: Ident,
    pub type_expression: TypeExpression,
}

pub struct StructDeclarator {
    pub identifier: Ident,
    pub parameters: Vec<StructParameter>,
}

pub struct StructParameter {
    pub identifier: Ident,
    pub is_owned: bool,
    pub type_expression: TypeExpression,
    pub default_value: Option<Expression>,
}

pub struct EnumDeclarator {
    pub identifier: Ident,
    pub values: Vec<Ident>,
}

pub enum Statement {
    ImportStatement(ImportStatement),
    InnerBlock(Block),
    IfStatement(IfStatement),
    LoopStatement(LoopStatement),
    JumpStatement(JumpStatement),
    Expression(Expression),
}

pub struct ImportStatement {
    pub is_reexport: bool,
    pub import_list: Vec<ImportIdentifier>,
    pub path: symbol::Symbol,
}

pub struct ImportIdentifier {
    pub old_name: Ident,
    pub new_name: Ident,
}

pub struct IfStatement {
    pub condition: Expression,
    pub true_block: Block,
    pub else_block: Option<Box<Conditional>>,
}

pub enum Conditional {
    If(IfStatement),
    Else(Block),
}

pub enum LoopStatement {
    ForLoop(ForLoop),
    ForInLoop(ForInLoop),
    WhileLoop(WhileLoop),
    DoWhileLoop(WhileLoop),
}

pub struct ForLoop {
    pub initial_expression: Option<Expression>,
    pub condition_expression: Option<Expression>,
    pub update_expression: Option<Expression>,
    pub block: Block,
}

pub struct ForInLoop {
    pub identifier: Ident,
    pub range: Expression,
    pub block: Block,
}

pub struct WhileLoop {
    pub condition: Expression,
    pub block: Block,
}

pub enum JumpStatement {
    Break,
    Continue,
    Return(Expression),
}

pub enum TypeExpression {
    PointerType(Box<TypeExpression>),
    SizedArrayType(SizedArrayType),
    UnsizedArrayType(Box<TypeExpression>),
    TypeUnion(TypeUnion),
    TypeofExpression(Expression),
    OptionalType(Box<TypeExpression>),
    NamedType(Ident),
}

pub struct SizedArrayType {
    pub size: Expression,
    pub type_expression: Box<TypeExpression>,
}

pub struct TypeUnion {
    pub first_type: Box<TypeExpression>,
    pub second_type: Box<TypeExpression>,
}

pub enum Expression {
    UnaryPrefixExpression(UnaryExpression),
    UnaryPostfixExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    TernaryExpression(TernaryExpression),
    PrimaryExpression(PrimaryExpression),
}

pub struct TernaryExpression {
    pub comparing_expression: Box<Expression>,
    pub true_case: Box<Expression>,
    pub false_case: Box<Expression>,
}

pub struct BinaryExpression {
    pub left_hand_side: Box<Expression>,
    pub operator: token::Token,
    pub right_hand_side: Box<Expression>,
}

pub struct UnaryExpression {
    pub expression: Box<Expression>,
    pub operator: token::Token,
}

pub enum PrimaryExpression {
    Literal(token::Token),
    Null,
    SubExpression(Box<Expression>),
    LambdaExpression(LambdaExpression),
    Reference(Reference),
}

pub struct LambdaExpression {
    pub parameters: Vec<Parameter>,
    pub block: Block,
}

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

pub struct MemberAccess {
    pub reference: Box<Reference>,
    pub member_ident: Ident,
}

pub struct FunctionCall {
    pub function: Box<Reference>,
    pub args: Vec<Expression>,
}

pub struct ConstructorCall {
    pub is_heap: bool,
    pub object_type: Box<Reference>,
    pub constructor_args: Option<Vec<Expression>>,
}

pub struct ArrayAccess {
    pub arr_reference: Box<Reference>,
    pub indices: Vec<Expression>,
}

pub struct CastReference {
    pub is_unsafe: bool,
    pub reference: Box<Reference>,
    pub new_type: Box<TypeExpression>,
}

pub struct Ident {
    pub name: symbol::Symbol,
}
