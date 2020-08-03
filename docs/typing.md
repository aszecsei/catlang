---
id: typing
title: Typing
sidebar_label: Typing
---

## Basic Types

### Arrays

Array types are prefixed with either `[]` or `[..]`, for static- or dynamically-sized arrays, respectively.

```catlang
let arr1: []int = [3]int;
let arr2: [..]int = [..]int;
```

There is also a type `Array<T>` for cases where either a static- or dynamically-sized array may be used:

```catlang
type Array<T> = []T | [..]T;
```

### Pointers

Pointer types are prefixed with a `*`:

```catlang
let i: *int = @myInt;
```

## Type Unions

Types can be composed through unions; this means that a value can be one of the listed types. For example:

```catlang
const falseIfEven = (num: int) -> bool | int {
  if (num % 2 == 0) {
    return false;
  }
  return num;
}
```

### Type Union Propagation

Most type operators (the pointer-to operator `*`, for example) propagate through type unions; that is, the following are equivalent:

```catlang
type a = *(x | y);
type b = *x | *y;
```

However, the array-of operator (`[]`) is not equivalent when propagated. It is, however, contained:

```catlang
type a = [](x | y); // not contained in b
type b = []x | []y; // contained in a
```

In this example, `b` is contained in `a`, but they are not equivalent:

```catlang
let c: [](float | int) = [] { 1, 2.4 }; // OK
let d: []float | []int = [] { 1, 2.4 }; // ERROR
```

## Type Definitions

Catlang allows users to write shorthands to refer to complex types. For example:

```catlang
type number = s8 | u8 | s16 | u16 | s32 | u32 | s64 | u64 | float | double;
const genericFunction = (input: number) -> {
  return input * 2;
}
```

## Any

The `any` type is the equivalent of C's `void*` type. It can be used to escape Catlang's type safety requirements.

## Optionals

Catlang includes an "optional" type, equivalent to `type | null`, to help avoid null-pointer exceptions. Any function that requires a non-Optional value must be enclosed in a conditional to ensure that the Optional value exists, or an error will be thrown.

```catlang
const myPrint = (num: int?) -> {
  if (num) {
    print(num);
  } else {
    print("Nope!");
  }
}
```

If a user wishes to force-unwrap an optional value, they can use the null-forgiving (`!`) operation to do so. This is not recommended as it may lead to null-pointer exceptions.

```catlang
const myPrint = (num: int?) -> {
  print(num!);
}
```

To reduce if-statements, null-coalescing and null-conditional operators are provided.

```catlang
const myPrint = (obj: MyStruct?) -> {
  if (obj) {
    obj.DoMethod();
  }
  // is equivalent to this using the null-conditional operator:
  obj?.DoMethod();

  if (!obj) {
    obj = defaultValue;
  }
  // is equivalent to this using the null coalescing assignment operator
  obj ??= defaultValue;
}
```

## Type Inference

### Function Arguments

Function arguments can not be inferred. Function return types, however, can be inferred.

```catlang
const timesTwo = (num) -> {
  return num * 2;
} // ERROR! Argument types cannot be inferred

const timesTwo = (num: int) -> {
  return num * 2;
} // OK!
```

### Variables

Variable types can be inferred, so long as an initial value is provided.

```catlang
const PI = 3.14159265; // OK!
let x; // ERROR!
let x: int; // OK!
```
