use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Declaration<'ast> {
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
    pub type_expression: Option<TypeExpressionNode<'ast>>,
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
    pub function_name: ScopedValueNode<'ast>,
    pub generic_parameters: IdentifierList<'ast>,
    pub parameters: NodeList<'ast, Parameter<'ast>>,
    pub return_type: Option<TypeExpressionNode<'ast>>,
    pub block: BlockNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parameter<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub type_expression: TypeExpressionNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StructDeclarator<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub generic_parameters: IdentifierList<'ast>,
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
    pub representation: TypeExpressionNode<'ast>, // TODO: Should this only be a primitive type?
    pub values: NodeList<'ast, EnumValue<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnumValue<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub value: Option<ExpressionNode<'ast>>,
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
