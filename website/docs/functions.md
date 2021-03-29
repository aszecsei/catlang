---
id: functions
title: Functions
sidebar_label: Functions
---

Functions can be defined in a C-like fashion:

```catlang
function timesTwo(num: int) -> int {
  return num * 2;
}
```

They can also be defined in a more functional style:

```catlang
const timesTwo = (num: int) -> int {
  return num * 2;
}
```

If a function does not return a value, the specified return type should be `void` (although, as with most return types, this can simply be inferred from the function itself).

```catlang
function doSomething() -> void {
  print("Hello, world!")
}
```

## Overloading

In catlang, a function defined via the `function` syntax may be overloaded:

```catlang
function printMe() -> void {
  print("Hello");
}
function printMe(times: int) -> void {
  for (i = 0; i < times; ++i) {
    printMe();
  }
}
```

A function declared using lambda-style syntax, may not be overloaded.

```catlang
const printMe = () -> {
  print("Hello");
}
const printMe = (times: int) -> { // ERROR: Re-definition of constant!
  for (i = 0; i < times; ++i) {
    printMe();
  }
}
```

## Global Functions

Global functions are defined without any namespace.

```catlang
const timesTwo = (num: int) -> {
  return num * 2;
}
const value = timesTwo(4);
```

## Static Functions

Static functions are attached to a struct, but not an instance of that struct. They _cannot_ use the keyword `this` in their parameter list.

```catlang
// C-style
function Vector3::dot(first: Vector3, second: Vector3) -> {
  return first.x * second.x + first.y * second.y + first.z * second.z;
}

// Functional-style
Vector3::dot = (first: Vector3, second: Vector3) -> {
  return first.x * second.x + first.y * second.y + first.z * second.z;
}

// Usage
const v1 = Vector3;
const v2 = Vector3;
const dot = Vector3::dot(v1, v2);
```

## Instance Functions

Instance functions are attached to an instance of a struct. They _must_ include the keyword `this` as their first parameter.

```catlang
// C-style
function Vector3::toString(this) -> {
  return "(${this.x}, ${this.y}, ${this.z})";
}

// Functional-style
Vector3::toString = (this) -> {
  return "(${this.x}, ${this.y}, ${this.z})";
}

// Usage
const v1 = Vector3;
print(v1.toString());
```

Functions attached to a struct, whether static or instance, are treated as immutable and cannot be redefined, only overloaded.
