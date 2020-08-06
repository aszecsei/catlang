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

## Properties

## Operators

### Subscript Operator

### Deconstruction Operator
