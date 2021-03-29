---
id: unions
title: Unions
sidebar_label: Unions
---

Types can be composed through unions; this means that a value can be one of the listed types. For example:

```catlang
const falseIfEven = (num: int) -> bool | int {
  if (num % 2 == 0) {
    return false;
  }
  return num;
}
```

## Type Union Propagation

Most type operators (the pointer-to operator `*`, for example) propagate through type unions; that is, the following are equivalent:

```catlang
type a = *(x | y);
type b = *x | *y;
```

However, the array-of operator (`[]`) is not equivalent when propagated. It is, instead, contained:

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
