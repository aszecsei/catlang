---
id: types
title: Types
sidebar_label: Types
---

Catlang is statically-typed; every expression has a type.

## Basic Data Types

### Integer Numbers

| Keyword        | Default Value | Description                      |
| -------------- | ------------- | -------------------------------- |
| `s8`           | 0             | signed 8 bits                    |
| `u8`           | 0             | unsigned 8 bits                  |
| `s16`          | 0             | signed 16 bits                   |
| `u16`          | 0             | unsigned 16 bits                 |
| `s32`          | 0             | signed 32 bits                   |
| `u32`          | 0             | unsigned 32 bits                 |
| `s64`          | 0             | signed 64 bits                   |
| `u64`          | 0             | unsigned 64 bits                 |
| `s128`         | 0             | signed 128 bits                  |
| `u128`         | 0             | unsigned 128 bits                |
| `char`         | 'xFF'         | unsigned 8 bit (UTF-8 code unit) |
| `short`        | 0             | equivalent to `s16`              |
| `int`          | 0             | equivalent to `s32`              |
| `long`         | 0             | equivalent to `s64`              |
| `c_short`      | 0             | for ABI compatibility with C     |
| `c_ushort`     | 0             | for ABI compatibility with C     |
| `c_int`        | 0             | for ABI compatibility with C     |
| `c_uint`       | 0             | for ABI compatibility with C     |
| `c_long`       | 0             | for ABI compatibility with C     |
| `c_ulong`      | 0             | for ABI compatibility with C     |
| `c_longlong`   | 0             | for ABI compatibility with C     |
| `c_ulonglong`  | 0             | for ABI compatibility with C     |
| `c_longdouble` | 0             | for ABI compatibility with C     |

:::note

Unless a type is specified, numbers are assumed to be the smallest type that can store the literal.

```catlang
const a = 128; // u8
const b = 128.0; // f32
const c = 123.; // f32
const d: f64 = 128; // f64
const e = 128 as f32; // f32
```

:::

### Booleans

| Keyword | Default Value | Description   |
| ------- | ------------- | ------------- |
| `bool`  | false         | boolean value |

### Floating-Point

| Keyword  | Default Value | Description           |
| -------- | ------------- | --------------------- |
| `f32`    | 0             | 32-bit floating point |
| `f64`    | 0             | 64-bit floating point |
| `float`  | 0             | equivalent to `f32`   |
| `double` | 0             | equivalent to `f64`   |

### Unvalued

| Keyword  | Default Value | Description                  |
| -------- | ------------- | ---------------------------- |
| `null`   | N/A           | null has no value            |
| `c_void` | N/A           | for ABI compatibility with C |

### Miscellaneous

| Keyword | Default Value | Description            |
| ------- | ------------- | ---------------------- |
| `type`  | N/A           | represents a type      |
| `any`   | N/A           | escape the type system |

## Derived Data Types

### Pointer

Pointer types are prefixed with a `*`:

```catlang
let i: *int = @myInt;
```

Pointers cannot be `null`. To use a null pointer, it must be of type `*T?`. The size of an optional pointer is guaranteed to be the same as the size of the pointer, and the value of the null pointer is guaranteed to be 0.

To retrieve the pointer to an object, use the `@` operator. To dereference a pointer, use the `*` operator.

Functions can be called on pointers the same way they can on standard objects - there's no need for C++'s arrow operator.

### Static Array

See [Arrays](arrays).

### Dynamic Array

See [Arrays](arrays).

### Functions

See [Functions](functions).

## User-Defined Types

1. [Enums](enums)
1. [Structs](structs)
1. [Unions](unions)
1. [Interfaces](interfaces)

## Type Conversions

### Integer Promotions

Integer promotions are conversions of the following types:

| from   | to    |
| ------ | ----- |
| `bool` | `s32` |
| `s8`   | `s32` |
| `u8`   | `s32` |
| `s16`  | `s32` |
| `u16`  | `s32` |
| `char` | `s32` |

If an enum has as a base type one of the types in the left column, it is converted to the type in the right column.

### Usual Arithmetic Conversions

The usual arithmetic conversions convert operands of binary operators to a common type. The operands must already be of arithmetic types. The following rules are applied in order, looking at the base type:

1. If either operand is `f64`, the other operand is converted to `f64`.
2. Else if either operand is `f32`, the other operand is converted to `f32`.
3. Else the integer promotions are done on each operand, followed by:
   1. If both are the same type, no more conversions are done.
   2. If both are signed or both are unsigned, the smaller type is converted to the larger.
   3. If the signed type is larger than the unsigned type, the unsigned type is converted to the signed type.
   4. The signed type is converted to the unsigned type.

## Aliased Types

### size_t

`size_t` is an alias to one of the unsigned integral basic types, and represents a type that is large enough to represent an offset into all addressable memory.

### ptrdiff_t

`ptrdiff_t` is an alias to the signed integral basic type the same size as `size_t`.

### Strings

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

## Type Modifiers

### Const Types

Types can be marked as const to indicate that they are immutable. This attribute is applied to any `const` declarations; for example, in

```catlang
const x: int = 0;
```

The type of `x` is `const int`. This normally has no impact when passing `x` to functions, as pass-by-value copying means that the `const` attribute can be safely removed:

```catlang
function timesTwo(a: int) -> int {
  return a * 2;
}

const x: int = 16;
let y = timesTwo(x);
```

However, if a pointer to `x` were to be passed, that type would be `*const int`, which would restrict modifications to `x`:

```catlang
function timesTwo(a: *int) -> {
  *a *= 2;
}

const x: int = 16;
timesTwo(@x); // ERROR!
```

### Volatile Types

The `volatile` type modifier indicates to the compiler that variables of this type may be modified by external sources, most commonly via memory-mapped I/O or multithreading.

## Type Definitions

Catlang allows users to write shorthands to refer to complex types. For example:

```catlang
type number = s8 | u8 | s16 | u16 | s32 | u32 | s64 | u64 | f32 | f64;
const genericFunction = (input: number) -> {
  return input * 2;
}
```

## Any

The `any` type is the equivalent of C's `void*` type. It can be used to escape Catlang's type system - `any` types can be cast to any type without any analysis. Note that this will _not_ perform any transformations on numeric types - a `float` cast to `any` and then cast to `int` will be an integer representation of the bits of that float, which is unlikely to be the desired behavior of such a casting operation. Be very careful when using this feature!

## Optionals

Catlang includes an "optional" type, equivalent to `type | null`, to help avoid null-pointer exceptions. Any function that requires a non-Optional value must be enclosed in a conditional to ensure that the Optional value exists, or an error will be thrown.

```catlang
const myPrint = (num: ?int) -> {
  if (num) {
    print(num);
  } else {
    print("Nope!");
  }
}
```

If a user wishes to force-unwrap an optional value, they can use the null-forgiving (`!`) operation to do so. This is not recommended as it may lead to null-pointer exceptions.

```catlang
const myPrint = (num: ?int) -> {
  print(num!);
}
```

To reduce if-statements, null-coalescing and null-conditional operators are provided.

```catlang
const myPrint = (obj: ?MyStruct) -> {
   // This:
  if (obj) {
    obj.DoMethod();
  }
  // is equivalent to this using the null-conditional operator:
  obj?.DoMethod();

  // And this:
  if (!obj) {
    obj = defaultValue;
  }
  // is equivalent to this using the null coalescing assignment operator
  obj ??= defaultValue;
}
```

## Tuples

> TODO

## Type Inference

Catlang does its best to infer type information where appropriate. For example, in this code

```catlang
let x = 3;
```

The type of the `x` variable is inferred to be `s32`. This kind of inference takes place when initializing variables, creating arrays, and determining function return types.

In most cases, type inference is straightforward. In the following sections, we'll explore some of the nuances in how type inference occurs.

### Best Common Type

When multiple types are in use, Catlang attempts to find a "best common type" for those types. This usually creates the smallest type union possible to encompass all values used. For example, in this code

```catlang
let x = [] { 1, 2, null };
```

The type of the `x` variable would be inferred to be `[](int | null)` - or, equivalently, `[](?int)`.

### Contextual Typing

In some cases, type inference can occur in the opposite direction - that is, the declared type influences the type of the expression. For example:

```catlang
function floatMap(fun: (f32) -> float, arr: []f32) -> []f32 {
  let res = new [arr.length]f32;
  for (i = 0; i < arr.length; ++i) {
    res[i] = fun(arr[i]);
  }
  return res;
}

floatMap((f) -> {
  return f * 2;
}, [] { 2.4, 3 })
```

Catlang is able to infer two things that it normally would not do: first, that the argument `f` in the anonymous function argument is a `f32` type, and second, that the value `3` in the array argument should be considered `f32` type.

Without the contextual information provided, Catlang would throw an error regarding the lack of type information for the `f` argument, and would interpret the array as being of type `[](int | float)`.
