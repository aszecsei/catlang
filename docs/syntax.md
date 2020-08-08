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
- `unreachable`
- `namespace`
- `using`

## Identifiers

Naming rules are similar to other programming languages. Identifiers start with either a letter or underscore and may contain letters, digits, and underscores. They are case-sensitive.

## Blocks

Curly braces (`{}`) define blocks. You can use a block anywhere a statement is allowed, such as in control flow statements. Function bodies are also blocks.

## Precedence and Associativity

| Precedence | Operator                                                                                                       | Description                                                             | Associativity |
| ---------- | -------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- | ------------- |
| 1          | `::`                                                                                                           | Scope resolution                                                        | Left-to-right |
| 2          | `a++` `a--` `a!` `a()` `a[]` `a.` `?`                                                                          | Unary suffix, function call, subscript, member access, null conditional | Left-to-right |
| 3          | `++a` `--a` `+a` `-a` `!a` `~a` `as` `@a` `*a` `??`                                                            | Unary prefix, casting, null coalesce                                    | Right-to-left |
| 4          | `*` `/` `%`                                                                                                    | Multiply, divide, modulo                                                | Left-to-right |
| 5          | `+` `-`                                                                                                        | Add, subtract                                                           | Left-to-right |
| 6          | `..` `...`                                                                                                     | Range                                                                   | Left-to-right |
| 7          | `<<` `>>`                                                                                                      | Bitshift                                                                | Left-to-right |
| 8          | `&`                                                                                                            | Bitwise and                                                             | Left-to-right |
| 9          | `^`                                                                                                            | Bitwise xor                                                             | Left-to-right |
| 10         | <code>&#124;</code>                                                                                            | Bitwise or                                                              | Left-to-right |
| 11         | `<` `<=` `>` `>=`                                                                                              | Comparison                                                              | Left-to-right |
| 12         | `is`                                                                                                           | Type test                                                               | Left-to-right |
| 13         | `==` `!=`                                                                                                      | Equality                                                                | Left-to-right |
| 14         | `&&`                                                                                                           | Logical and                                                             | Left-to-right |
| 15         | <code>&#124;&#124;</code>                                                                                      | Logical or                                                              | Left-to-right |
| 16         | `a ? b : c`                                                                                                    | Ternary                                                                 | Right-to-left |
| 17         | `=` `+=` `-=` `*=` `/=` `%=` `&=` `&&=` <code>&#124;=</code> <code>&#124;&#124;=</code> `^=` `<<=` `>>=` `??=` | Assignment                                                              | Right-to-left |
