---
id: native-types
title: Native Types
sidebar_label: Native Types
---

## Primitives

1. Integer numbers
   1. `S8` (1 byte, signed)
   1. `U8` (1 byte, unsigned)
   1. `S16` (2 bytes, signed)
   1. `U16` (2 bytes, unsigned)
   1. `S32` (4 bytes, signed)
   1. `U32` (4 bytes, unsigned)
   1. `S64` (8 bytes, signed)
   1. `U64` (8 bytes, unsigned)
   1. `char` (equivalent to `U8`)
   1. `short` (equivalent to `S16`)
   1. `int` (equivalent to `S32`)
   1. `long` (equivalent to `S64`)
1. Booleans
   1. `bool` (1 byte)
1. Floating-Point Numbers
   1. `float` (4 bytes)
   1. `double` (8 bytes)
1. Unvalued
   1. `null`

### Number Literals

Unless a type is specified, numbers are assumed to be the smallest type that can store the literal.

```
const a = 128; // U8
const b = 128.0; // float
const c = 123.; // float
const d: float = 128; // float
const e = 128 as float; // float
```

### String Literals

Catlang uses the C-style convention that character literals are wrapped with single quotes, while string literals are wrapped with double quotes:

```
const a = 'a'; // char
const b = "a"; // string
const c = "abc"; // string
const d = 'abc'; // ERROR!
```

## Type Definitions

Catlang allows users to write shorthands to refer to complex types. For example:

```
type number = S8 | U8 | S16 | U16 | S32 | U32 | S64 | U64 | float | double;
const genericFunction = (input: number) -> {
  return input * 2;
}
```

## Any

The `any` type is the equivalent of C's `void*` type. It can be used to escape Catlang's type safety requirements.

## Optionals

Catlang includes an "optional" tye, equivalent to `type | null`, to help avoid null-pointer exceptions. Any function that requires a non-Optional value must be enclosed in a conditional to ensure that the Optional value exists, or an error will be thrown.

```
const myPrint = (num?: int) -> {
  if (num) {
    print(num);
  } else {
    print("Nope!");
  }
}
```

If a user wishes to force-unwrap an optional value, they can use the `!` operation to do so. This is not recommended as it may lead to null-pointer exceptions.

```
const myPrint = (num?: int) -> {
  print(num!);
}
```

## Collections

### Arrays

Catlang's arrays differ from C-style arrays in that they contain information about their length (see [C's Biggest Mistake](http://www.drdobbs.com/architecture-and-design/cs-biggest-mistake/228701625)).

```
const finalElement = myArray[myArray.length - 1];
```

Arrays also include convenience methods for a more functional style of programming: namely `map`, `filter`, `each`, and `reduce`.

Arrays are instantiated using a syntax similar to C, and can be either static- or dynamically-sized:

```
const staticArray = [25]int;
const dynamicArray = [..]int;
```

Arrays of pointers and pointers to arrays are syntactically different:

```
const pointerToArray : *[]int = @myArray;
const arrayOfPointers : []*int = myArray;
```

Arrays can be iterated over:

```
const arr = []int { 1, 2, 3, 4, 5 };
for (x in arr) {
  print(x);
}
```

They can also iterate over inner properties of structs; in the following example, loops A and B are equivalent:

```
struct Vector3 {
  x: float;
  y: float;
  z: float;
}
const arr = []Vector3 { ... };
for (xVal in arr.x) {
  print(xVal);
} // A
for (vec in arr) {
  print(vec.x);
} // B
```

### Strings

Catlang's `string` type is a wrapper around `[]char`. A notable feature of the language is string interpolation:

```
let name = "";
print("Enter your name: ");
readLine(name);
print("Hello, ${name}!\n");
```

### Structs

Catlang's syntax for defining structures is straightforward:

```
struct Vector3 {
  x: float;
  y: float;
  z: float;
}
```

Structs can also have default values assigned:

```
struct Vector3 {
  x: float = 1.0;
  y: float = 1.0;
  z: float = 1.0;
}
```

To denote an "owned" property in a struct, use the `owned` keyword. When a struct is deleted, all owned properties are also deleted.

```
struct Transform {
  owned position: Vector3;
  owned rotation: Quaternion;
}
```

To allocate memory on the stack, users should declare variables _without_ the `new` keyword. If the `new` keyword is used, the variable is allocated on the heap and must be freed later with the `delete` keyword.

```
const v1 = Vector3; // Allocated on the stack.
const v2 = new Vector3; // Allocated on the heap...
delete v2; // ...so must be manually freed.
```

Structs can define how they are stored in arrays, in order to reduce cache misses. Structs default to the "array of structs" schema, but can be swapped to the "struct of arrays" schema using the `SOA` keyword.

```
struct V3A {
  x : float = 1;
  y : float = 2;
  z : float = 3;
}
let v1 = [4]V3A; // Memory will contain 1 2 3 1 2 3 1 2 3 1 2 3
struct V3B SOA {
  x : float = 1;
  y : float = 2;
  z : float = 3;
}
let v2 = [4]V3B; // Memory will contain 1 1 1 1 2 2 2 2 3 3 3 3
```

No matter how these arrays are stored in memory, they are used and referenced the same way within Catlang code.

Fields of a struct that begin with an underscore are private fields and are only accessible to functions within the struct's namespace. All other fields are public.

```
struct MyStruct {
  _hiddenVar: string;
  publicVar: string;
}
```

## Pointers

To retrieve the pointer to an object, use the `@` operator. To dereference a pointer, use the `*` operator.

Functions can be called on pointers the same way they can on standard objects - there's no need for C++'s arrow operator.

## Enums

```
enum Alignment {
  Left, // 0
  Right, // 1
  Center, // 2
}
const a = Alignment::Left;
const val = a as U16;
```

```
enum Alignment: U32 {
  Left, // 0
  Right = 12, // 12
  Center, // 13
}
```
