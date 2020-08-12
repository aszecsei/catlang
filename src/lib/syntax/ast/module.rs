use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Module<'ast> {
    pub elements: NodeList<'ast, ModuleElement<'ast>>,
    pub is_script: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModuleElement<'ast> {
    Import(ImportNode<'ast>),
    Export(ExportNode<'ast>),
    Declaration(DeclarationNode<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Import<'ast> {
    pub import_list: ImportList<'ast>,
    pub path: StringLiteralNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImportList<'ast> {
    NamedImportList(NamedImportList<'ast>),
    GlobImportList(GlobImportList<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NamedImportList<'ast> {
    pub imports: NodeList<'ast, ImportIdentifier<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobImportList<'ast> {
    pub identifier: IdentifierNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImportIdentifier<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub renamed_to: Option<IdentifierNode<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Export<'ast> {
    Declaration(DeclarationNode<'ast>),
    Statement(ExportStatement<'ast>),
    ReExport(ExportReExport<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExportStatement<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub renamed_to: Option<IdentifierNode<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExportReExport<'ast> {
    pub exports: ImportList<'ast>,
    pub path: StringLiteralNode<'ast>,
}

pub type ImportNode<'ast> = Node<'ast, Import<'ast>>;
pub type ExportNode<'ast> = Node<'ast, Export<'ast>>;

impl_from! {
    ImportNode => ModuleElement::Import,
    ExportNode => ModuleElement::Export,
    DeclarationNode => ModuleElement::Declaration,

    NamedImportList => ImportList::NamedImportList,
    GlobImportList => ImportList::GlobImportList,

    DeclarationNode => Export::Declaration,
    ExportStatement => Export::Statement,
    ExportReExport => Export::ReExport,
}
