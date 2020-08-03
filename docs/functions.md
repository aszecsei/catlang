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

This is syntactically equivalent to a second, more functional approach:

```catlang
const timesTwo = (num: int) -> int {
  return num * 2;
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
Vector3::dot = (first: Vector3, second: Vector3) -> {
  return first.x * second.x + first.y * second.y + first.z * second.z;
}
const v1 = Vector3;
const v2 = Vector3;
const dot = Vector3::dot(v1, v2);
```

## Instance Functions

Instance functions are attached to an instance of a struct. They _must_ include the keyword `this` as their first parameter.

```catlang
Vector3::toString = (this) -> {
  return "(${this.x}, ${this.y}, ${this.z})";
  const v1 = Vector3;
  print(v1.toString());
}
```

Functions attached to a struct, whether static or instance, are treated as immutable and cannot be redefined, only overloaded.
