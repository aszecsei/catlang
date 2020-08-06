---
id: syntax
title: Syntax
sidebar_label: Syntax
---

## Comments

Line comments start with `//` and end at the end of the line:

```catlang
// This is a comment.
```

Block comments start with `/*` and end with `*/`. They can span multiple lines:

```catlang
/* This
   is
   a
   multi-line
   comment. */
```

Unlike C, block comments can nest in catlang:

```catlang
/* this is /* a nested */ comment. */
```

Thus, you can easily comment out an entire block of code, even if the code already contains block comments.

## Reserved Words

- `any`
- `let`
- `const`
- `new`
- `delete`
- `typeof`
- `is`
- `as`
- `in`
- `function`
- `return`
- `struct`
- `type`
- `enum`
- `owned`
- `import`
- `export`
- `from`
- `for`
- `while`
- `do`
- `loop`
- `if`
- `else`
- `break`
- `continue`
- `null`
- `true`
- `false`
- `this`
- `volatile`

## Identifiers

Naming rules are similar to other programming languages. Identifiers start with either a letter or underscore and may contain letters, digits, and underscores. They are case-sensitive.

## Blocks

Curly braces (`{}`) define blocks. You can use a block anywhere a statement is allowed, such as in control flow statements. Function bodies are also blocks.

## Precedence and Associativity

> TODO
