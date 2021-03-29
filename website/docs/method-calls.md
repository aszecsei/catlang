---
id: method-calls
title: Method Calls
sidebar_label: Method Calls
---

Methods are performed using a traditional syntax:

```catlang
myFunction(arguments);
```

## Instance Method Calls

```catlang
const v1 = Vector3;
const v2 = v1.double();
```

## Static Method Calls

```catlang
const v1 = Vector3;
const v2 = Vector3;
const dot = Vector3::dot(v1, v2);
```

## Operators

Catlang supports operator overloading for a variety of arithmetic operators:

```catlang
struct Vector2 {
  x: float,
  y: float
}
function Vector2::operator +(this, rhs: Vector2) -> Vector2 {
  return Vector2 {
    x: lhs.x + rhs.x,
    y: lhs.y + rhs.y
  };
}

let v1: Vector2;
let v2: Vector2;
let v3 = v1 + v2;
```

Operator overloads _cannot_ be static functions.

### Subscript Operator

In addition to common operators, catlang also supports overloading the `[]` subscript operator. This allows developers to create custom collections that can be accessed just like arrays.

```catlang
struct Vector2 {
  x: float,
  y: float
}

function Vector2::operator [](this, index: int) -> float {
  if (index == 0) return v.x;
  else return v.y;
}
function Vector2::operator []=(this, index: int, value: float) {
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

function Vector2::operator deconstruct(this) -> (float, float) {
  return (v.x, v.y);
}

let example = Vector2 { x: 1.0, y: 2.0 };
let (a, b) = example; // a is 1.0, b is 2.0
```

### Casting Operator

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
