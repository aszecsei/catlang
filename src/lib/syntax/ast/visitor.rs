use tracing::info;

use crate::syntax::ast::Identifier;

pub trait Visitor {
    fn visit_identifier(&mut self, identifier: Identifier) -> anyhow::Result<()> {
        Ok(())
    }
}

pub trait Visitable {
    fn visit(&self, v: &mut dyn Visitor) -> anyhow::Result<()>;
}

impl<T: Visitable> Visitable for Option<T> {
    #[inline]
    fn visit(&self, v: &mut dyn Visitor) -> anyhow::Result<()> {
        match self {
            Some(node) => node.visit(v),
            None => Ok(()),
        }
    }
}

pub struct PrintVisitor;

impl Visitor for PrintVisitor {
    fn visit_identifier(&mut self, identifier: Identifier) -> anyhow::Result<()> {
        info!("Identifier: {}", identifier);
        Ok(())
    }
}
