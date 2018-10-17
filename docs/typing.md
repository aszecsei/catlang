---
id: Typing
title: Typing
sidebar_label: Typing
---

Catlang uses strong typing, although most typing can be inferred.

## Function Arguments

Function arguments can not be inferred. Function return types, however, can be inferred.

```
const timesTwo = (num) -> {
  return num * 2;
} // ERROR! Argument types cannot be inferred

const timesTwo = (num: int) -> {
  return num * 2;
} // OK!
```

## Variables

Variable types can be inferred, so long as an initial value is provided.

```
const PI = 3.14159265; // OK!
let x; // ERROR!
let x: int; // OK!
```

## Type Unions

Types can be composed through unions; this means that a value can be one of the listed types. For example:

```
const falseIfEven = (num: int) -> bool | int {
  if (num % 2 == 0) {
    return false;
  }
  return num;
}
```
