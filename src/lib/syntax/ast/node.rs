use crate::syntax::ast::{Visitable, Visitor};
use std::fmt::{self, Debug};
use std::ops::Deref;
use toolshed::CopyCell;

pub trait OptionalLocation {
    fn start(&self) -> Option<u32>;
    fn end(&self) -> Option<u32>;
}

impl<'ast, T> OptionalLocation for Option<Node<'ast, T>> {
    #[inline]
    fn start(&self) -> Option<u32> {
        (*self).as_ref().map(|node| node.start)
    }

    #[inline]
    fn end(&self) -> Option<u32> {
        (*self).as_ref().map(|node| node.end)
    }
}

/// `Node` is a specialized `Cell` that holds a reference to T instead of T.
/// `Node` has defined lifetime and implements `Defer<Target = T>` for convenience.
#[derive(Clone, Copy)]
pub struct Node<'ast, T: 'ast> {
    inner: CopyCell<&'ast NodeInner<T>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeInner<T> {
    pub start: u32,
    pub end: u32,
    pub value: T,
}

impl<T> NodeInner<T> {
    #[inline]
    pub fn new(start: u32, end: u32, value: T) -> Self {
        NodeInner { start, end, value }
    }
}

impl<'ast, T: 'ast> Node<'ast, T> {
    #[inline]
    pub fn new(ptr: &'ast NodeInner<T>) -> Self {
        Node {
            inner: CopyCell::new(ptr),
        }
    }

    #[inline]
    pub fn set(&self, ptr: &'ast NodeInner<T>) {
        self.inner.set(ptr)
    }
}

impl<'ast, T: 'ast> Deref for Node<'ast, T> {
    type Target = NodeInner<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.get()
    }
}

impl<'ast, T: 'ast + PartialEq> PartialEq for Node<'ast, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<'ast, T: 'ast + Debug> Debug for Node<'ast, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<'ast, T: 'ast + Visitable> Visitable for Node<'ast, T> {
    #[inline]
    fn visit(&self, v: &mut dyn Visitor) -> anyhow::Result<()> {
        self.inner.get().value.visit(v)
    }
}

impl<T: Debug> Debug for NodeInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{}) ", self.start, self.end)?;

        Debug::fmt(&self.value, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ptr() {
        let node1 = NodeInner::new(0, 0, "foo");
        let node2 = NodeInner::new(0, 0, "bar");

        let node1_ptr = Node::new(&node1);
        let node2_ptr = node1_ptr;

        assert_eq!(*node1_ptr, NodeInner::new(0, 0, "foo"));
        assert_eq!(*node2_ptr, NodeInner::new(0, 0, "foo"));

        node2_ptr.set(&node2);

        assert_eq!(*node1_ptr, NodeInner::new(0, 0, "foo"));
        assert_eq!(*node2_ptr, NodeInner::new(0, 0, "bar"));
    }
}
