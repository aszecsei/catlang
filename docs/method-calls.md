---
id: method-calls
title: Method Calls
sidebar_label: Method Calls
---

## Signature

In catlang, a function defined via the `function` syntax may be overloaded:

```catlang
function printMe() -> {
  print("Hello");
}
function printMe(times: int) -> {
  for (i = 0; i < times; ++i) {
    printMe();
  }
}
```

A function declared using lambda-style syntax, may not be overloaded.

```catlang
const printMe = () => {
  print("Hello");
}
const printMe = (times: int) -> { // ERROR: Re-definition of constant!
  for (i = 0; i < times; ++i) {
    printMe();
  }
}
```

## Operators

Catlang supports operator overloading for a variety of arithmetic operators:

```catlang
struct Vector2 {
  x: float,
  y: float
}
function Vector2::operator +(lhs: Vector2, rhs: Vector2) -> Vector2 {
  return Vector2 {
    x: lhs.x + rhs.x,
    y: lhs.y + rhs.y
  };
}
```

Operator overloads _must_ be static functions.

### Subscript Operator

In addition to common operators, catlang also supports overloading the `[]` subscript operator. This allows developers to create custom collections that can be accessed just like arrays.

```catlang
struct Vector2 {
  x: float,
  y: float
}

function Vector2::operator [](v: Vector2, index: int) -> float {
  if (index == 0) return v.x;
  else return v.y;
}
function Vector2::operator []=(v: Vector2, index: int, value: float) {
  if (index == 0) v.x = value;
  else v.y = value;
}

let example = Vector2 { x: 1.0, y: 2.0 };
let xval = example[0]; // 1.0
example[1] = 3.0; // Vector2 { x: 1.0, y: 3.0 };
```

### Deconstruction Operator

Catlang allows deconstruction of structs that implement the `deconstruct` operator.

```catlang
struct Vector2 {
  x: float,
  y: float
}

function Vector2::operator deconstruct(v: Vector2) -> (float, float) {
  return (v.x, v.y);
}

let example = Vector2 { x: 1.0, y: 2.0 };
let (a, b) = example; // a is 1.0, b is 2.0
```

## Casting Operator

To create a user-defined cast, you can overload the casting operator:

```catlang
struct Vector2 {
  x: float,
  y: float
}
struct Vector3 {
  x: float,
  y: float,
  z: float
}

function Vector3::operator Vector2(v: Vector3) -> Vector2 {
  return Vector2 {
    x: v.x,
    y: v.y
  };
}

let x = Vector3 {
  x: 1.0,
  y: 2.0,
  z: 3.0
};
let y = x as Vector2; // Vector2 { x: 1.0, y: 2.0 }
```
