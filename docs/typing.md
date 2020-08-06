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

This has some implications for implicit casting to type unions:

```catlang
function sum1(arr: [](int | float)) -> float {
  let res: float = 0;
  for (num in arr) {
    res += num as float;
  }
  return res;
}

let arr1 = []float { 2.4, 1.2 };
let arr1sum = sum1(arr1); // OK

function sum2(arr: []int | []float) -> float {
  let res: float = 0;
  for (num in arr) {
    res += num as float;
  }
  return res;
}
let arr2 = [](int | float) { 2, 2.4 };
let arr2sum = sum2(arr2); // ERROR: Array is not either an array of floats or an array of ints
```

The typically-desired behavior is the former. This behavior is what occurs when a developer creates a type definition for a type union and then creates an array of that type definition; this is the recommended practice, to avoid unintended behavior.

## Type Definitions

Catlang allows users to write shorthands to refer to complex types. For example:

```catlang
type number = s8 | u8 | s16 | u16 | s32 | u32 | s64 | u64 | float | double;
const genericFunction = (input: number) -> {
  return input * 2;
}
```

## Any

The `any` type is the equivalent of C's `void*` type. It can be used to escape Catlang's type system - `any` types can be cast to any type without any analysis. Note that this will _not_ perform any transformations on numeric types - a `float` cast to `any` and then cast to `int` will be an integer representation of the bits of that float, which is unlikely to be the desired behavior of such a casting operation. Be very careful when using this feature!

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

Catlang does its best to infer type information where appropriate. For example, in this code

```catlang
let x = 3;
```

The type of the `x` variable is inferred to be `int`. This kind of inference takes place when initializing variables, creating arrays, and determining function return types.

In most cases, type inference is straightforward. In the following sections, we'll explore some of the nuances in how type inference occurs.

### Best Common Type

When multiple types are in use, Catlang attempts to find a "best common type" for those types. This usually creates the smallest type union possible to encompass all values used. For example, in this code

```catlang
let x = [] { 1, 2, null };
```

The type of the `x` variable would be inferred to be `[](int | null)` - or, equivalently, `[](int?)`.

### Contextual Typing

In some cases, type inference can occur in the opposite direction - that is, the declared type influences the type of the expression. For example:

```catlang
function floatMap(fun: (float) -> float, arr: []float) -> []float {
  let res = new [arr.length]float;
  for (i = 0; i < arr.length; ++i) {
    res[i] = fun(arr[i]);
  }
  return res;
}

floatMap((f) -> {
  return f * 2;
}, [] { 2.4, 3 })
```

Catlang is able to infer two things that it normally would not do: first, that the argument `f` in the anonymous function argument is a `float` type, and second, that the value `3` in the array argument should be considered `float` type.

Without the contextual information provided, Catlang would throw an error regarding the lack of type information for the `f` argument, and would interpret the array as being of type `[](int | float)`.
