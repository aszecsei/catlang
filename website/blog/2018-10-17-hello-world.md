---
title: Hello, World!
author: Alic Szecsei
authorURL: https://alic-szecsei.com/
authorImageURL: https://alic-szecsei.com/img/profile2.png
---

While much of Catlang is progressing nicely (the grammar is almost entirely fleshed out!) there have been a few setbacks. Namely, we're switching from Go to Rust - which is a net positive, but requires re-writing some of the existing code.

<!--truncate-->

## Why Switch?

This mostly has to do with the AST. Go has some strange notions with regards to inheritance - namely, that interfaces are used exclusively. So in order to create a type of `Declaration`, we have to construct an interface `Declaration` which requires its members to implement a function `declaration()`; then all "subclasses" of `Declaration` just implement that empty function.

Not optimal, and as we progress further into the parser and IR, these shortcomings are going to become more and more of a problem. So we're cutting the problem off at the pass and using Rust; it has the features of Go that were very helpful (package management, unicode support) and keeps a more traditional, useful inheritance model.
