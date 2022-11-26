#[macro_use]
mod impl_from;

mod attribute;
mod block;
mod declaration;
mod expression;
mod module;
mod node;
mod source;
mod statement;
mod types;
mod visitor;

use std::marker::PhantomData;
use toolshed::list::{List, UnsafeList};
use toolshed::Arena;

pub use self::attribute::*;
pub use self::block::*;
pub use self::declaration::*;
pub use self::expression::*;
pub use self::module::*;
pub use self::node::{Node, NodeInner, OptionalLocation};
pub use self::source::*;
pub use self::statement::*;
pub use self::types::*;
pub use self::visitor::*;

/// Useful for boolean flags that need location information via FlagNode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flag;

pub type Identifier<'ast> = &'ast str;
pub type StringLiteral<'ast> = &'ast str;

pub type FlagNode<'ast> = Node<'ast, Flag>;
pub type NodeList<'ast, T> = List<'ast, Node<'ast, T>>;
pub type SourceUnitNode<'ast> = Node<'ast, SourceUnit<'ast>>;
pub type SourceUnitList<'ast> = NodeList<'ast, SourceUnit<'ast>>;
pub type IdentifierNode<'ast> = Node<'ast, Identifier<'ast>>;
pub type IdentifierList<'ast> = NodeList<'ast, Identifier<'ast>>;
pub type StringLiteralNode<'ast> = Node<'ast, StringLiteral<'ast>>;

impl<'ast, T: 'ast + Visitable> Visitable for NodeList<'ast, T> {
    #[inline]
    fn visit(&self, v: &mut dyn Visitor) -> anyhow::Result<()> {
        for node in self {
            node.visit(v)?;
        }
        Ok(())
    }
}

/// A catlang source code parsed to an AST
pub struct Program<'ast> {
    /// `SourceUnitList<'ast>` converted to an `UnsafeList` to deal with
    /// the fact that the `Arena` on which it lives is also in this struct.
    body: UnsafeList,
    /// `Arena` on which the entire AST is allocated.
    arena: Arena,
    /// For lifetime safety :)
    _phantom: PhantomData<SourceUnitList<'ast>>,
}

impl<'ast> Program<'ast> {
    #[inline]
    pub fn new(body: UnsafeList, arena: Arena) -> Self {
        Program {
            body,
            arena,
            _phantom: PhantomData,
        }
    }

    /// Get the list of `SourceUnit`s.
    #[inline]
    pub fn body(&self) -> SourceUnitList<'ast> {
        unsafe { self.body.into_list() }
    }

    /// Get a reference to the `Arena` on which the AST is allocated.
    #[inline]
    pub fn arena(&'ast self) -> &'ast Arena {
        &self.arena
    }
}
