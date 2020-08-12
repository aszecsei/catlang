use crate::syntax::ast::*;

/// A `SourceUnit` is the top level construct of the grammar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SourceUnit<'ast> {
    Module(Module<'ast>),
}

impl_from! {
    Module => SourceUnit::Module,
}

impl Visitable for SourceUnit<'_> {
    fn visit(&self, v: &mut dyn Visitor) -> anyhow::Result<()> {
        unimplemented!()
    }
}
