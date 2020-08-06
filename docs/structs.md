---
id: structs
title: Structs
sidebar_label: Structs
---

## Defining a Struct

Catlang's syntax for defining structures is straightforward:

```catlang
struct Vector3 {
  x: float;
  y: float;
  z: float;
}
```

Structs can also have default values assigned:

```catlang
struct Vector3 {
  x: float = 1.0;
  y: float = 1.0;
  z: float = 1.0;
}
```

To denote an "owned" pointer in a struct, use the `owned` keyword. When a struct is deleted, all owned pointers are also deleted.

```catlang
struct Transform {
  owned position: *Vector3;
  owned rotation: *Quaternion;
}
```

To allocate memory on the stack, users should declare variables _without_ the `new` keyword. If the `new` keyword is used, the variable is allocated on the heap and must be freed later with the `delete` keyword.

```catlang
const v1 = Vector3; // Allocated on the stack.
const v2 = new Vector3; // Allocated on the heap...
delete v2; // ...so must be manually freed.
```

Structs can define how they are stored in arrays, in order to reduce cache misses. Structs default to the "array of structs" schema, but can be swapped to the "struct of arrays" schema using the `SOA` attribute.

```catlang
struct V3A {
  x : float = 1;
  y : float = 2;
  z : float = 3;
}
let v1 = [4]V3A; // Memory will contain 1 2 3 1 2 3 1 2 3 1 2 3

#[SOA]
struct V3B {
  x : float = 1;
  y : float = 2;
  z : float = 3;
}
let v2 = [4]V3B; // Memory will contain 1 1 1 1 2 2 2 2 3 3 3 3
```

No matter how these arrays are stored in memory, they are used and referenced the same way within Catlang code.

Fields of a struct that begin with an underscore are private fields and are only accessible to functions within the struct's namespace. All other fields are public.

```catlang
struct MyStruct {
  _hiddenVar: string;
  publicVar: string;
}
```

## Attributes

Attributes are built-in language features. Future support for custom attributes may be added in future. They modify the value defined immediately afterwards:

```catlang
#[SOA]
struct V3 {
  x : float = 1;
  y : float = 2;
  z : float = 3;
}
```
