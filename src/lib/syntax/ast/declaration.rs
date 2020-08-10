use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Declaration<'ast> {
    pub is_exported: bool,
    pub attributes: AttributeList<'ast>,
    pub declarator: Declarator<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Declarator<'ast> {
    Constant(ConstantDeclarator<'ast>),
    Type(TypeDeclarator<'ast>),
    Variable(VariableDeclarator<'ast>),
    Function(FunctionDeclarator<'ast>),
    Struct(StructDeclarator<'ast>),
    Enum(EnumDeclarator<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstantDeclarator<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub expression: ExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeDeclarator<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub type_expression: TypeExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VariableDeclarator<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub type_expression: Option<TypeExpressionNode<'ast>>,
    pub expression: Option<ExpressionNode<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionDeclarator<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub parameters: NodeList<'ast, Parameter<'ast>>,
    pub return_type: Option<TypeExpressionNode<'ast>>,
    pub block: BlockNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parameter<'ast> {
    pub is_const: bool,
    pub identifier: IdentifierNode<'ast>,
    pub type_expression: TypeExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StructDeclarator<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub members: NodeList<'ast, StructMember<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StructMember<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub is_owned: bool,
    pub type_expression: TypeExpressionNode<'ast>,
    pub default_value: Option<ExpressionNode<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnumDeclarator<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub type_expression: Option<TypeExpressionNode<'ast>>,
    pub values: IdentifierList<'ast>,
}

pub type DeclarationNode<'ast> = Node<'ast, Declaration<'ast>>;

impl_from! {
    ConstantDeclarator => Declarator::Constant,
    TypeDeclarator => Declarator::Type,
    VariableDeclarator => Declarator::Variable,
    FunctionDeclarator => Declarator::Function,
    StructDeclarator => Declarator::Struct,
    EnumDeclarator => Declarator::Enum,
}
