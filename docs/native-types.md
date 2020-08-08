---
id: native-types
title: Native Types
sidebar_label: Native Types
---

## Primitives

1. Integer numbers
   1. `s8` (1 byte, signed)
   1. `u8` (1 byte, unsigned)
   1. `s16` (2 bytes, signed)
   1. `u16` (2 bytes, unsigned)
   1. `s32` (4 bytes, signed)
   1. `u32` (4 bytes, unsigned)
   1. `s64` (8 bytes, signed)
   1. `u64` (8 bytes, unsigned)
   1. `char` (equivalent to `u8`)
   1. `short` (equivalent to `s16`)
   1. `int` (equivalent to `s32`)
   1. `long` (equivalent to `s64`)
   1. `c_short` (for ABI compatibility with C)
   1. `c_ushort` (for ABI compatibility with C)
   1. `c_int` (for ABI compatibility with C)
   1. `c_uint` (for ABI compatibility with C)
   1. `c_long` (for ABI compatibility with C)
   1. `c_ulong` (for ABI compatibility with C)
   1. `c_longlong` (for ABI compatibility with C)
   1. `c_ulonglong` (for ABI compatibility with C)
   1. `c_longdouble` (for ABI compatibility with C)
1. Booleans
   1. `bool` (1 byte)
1. Floating-Point Numbers
   1. `f32` (4 bytes)
   1. `f64` (8 bytes)
   1. `float` (equivalent to `f32`)
   1. `double` (equivalent to `f64`)
1. Unvalued
   1. `null`
   1. `noreturn`
   1. `c_void` (for ABI compatibility with C)
1. Miscellaneous
   1. `type`

## Number Literals

Unless a type is specified, numbers are assumed to be the smallest type that can store the literal.

```catlang
const a = 128; // U8
const b = 128.0; // float
const c = 123.; // float
const d: float = 128; // float
const e = 128 as float; // float
```

## String Literals

Catlang uses the C-style convention that character literals are wrapped with single quotes, while string literals are wrapped with double quotes:

```catlang
const a = 'a'; // char
const b = "a"; // string
const c = "abc"; // string
const d = 'abc'; // ERROR!
```

A notable feature of the language is string interpolation:

```catlang
let name = "";
print("Enter your name: ");
readLine(name);
print("Hello, ${name}!\n");
```

## Pointers

To retrieve the pointer to an object, use the `@` operator. To dereference a pointer, use the `*` operator.

Functions can be called on pointers the same way they can on standard objects - there's no need for C++'s arrow operator.
