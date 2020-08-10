use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Module<'ast> {
    pub elements: NodeList<'ast, ModuleElement<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Script<'ast> {
    pub elements: NodeList<'ast, ScriptElement<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModuleElement<'ast> {
    Import(Import<'ast>),
    Export(Export<'ast>),
    Declaration(Declaration<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScriptElement<'ast> {
    Declaration(Declaration<'ast>),
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
    pub renamed_to: IdentifierNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Export<'ast> {
    Declaration(Declaration<'ast>),
    Statement(ExportStatement<'ast>),
    ReExport(ExportReExport<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExportStatement<'ast> {
    pub identifier: IdentifierNode<'ast>,
    pub renamed_to: IdentifierNode<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExportReExport<'ast> {
    pub exports: NodeList<'ast, ImportIdentifier<'ast>>,
    pub path: StringLiteralNode<'ast>,
}

pub type ImportNode<'ast> = Node<'ast, Import<'ast>>;

impl_from! {
    Import => ModuleElement::Import,
    Export => ModuleElement::Export,
    Declaration => ModuleElement::Declaration,

    Declaration => ScriptElement::Declaration,

    NamedImportList => ImportList::NamedImportList,
    GlobImportList => ImportList::GlobImportList,

    Declaration => Export::Declaration,
    ExportStatement => Export::Statement,
    ExportReExport => Export::ReExport,
}
