---
id: casting
title: Casting
sidebar_label: Casting
---

Casting is a way to check the type of an instance or to treat an instance of one type as another. There is no implicit casting in Catlang.

## Type Checking

Type checking is performed with the `is` operator:

```
const x: int | string = otherFunction();
if (x is int) {
  print("x is an int!");
} else {
  print("x is a string!");
}
```

## Safe Casting

Safe casting returns an optional type, equivalent to `type | null`; safe casting must be evaluated at run-time. If run-time type information indicates that the variable's _actual value_ is of the casted type, the cast will succeed. Otherwise, the cast will return `null`.

```
const x: int | string = otherFunction();
const myFunc = (i: int) -> {
  // Something
}

myFunc(x); // ERROR!
myFunc(x as int); // ERROR!
let castX = x as int;
if (castX) {
  myFunc(castX); // OK
}
```

Since an assignment operator returns the object that was assigned, the previous example can be condensed:

```
if (let castX = x as int) {
  myFunc(castX); // OK
}
```

This has the twofold benefit of being more concise and scoping the `castX` variable to the `if` block.

Internally, numbers can be safely cast to any number type that is guaranteed not to lose data. For example:

```
const char = 'a';
const xInt = xChar as int; // OK!
const yInt = -2;
const yChar = yInt as char; // ERROR!
```

## Unsafe Casting

Unsafe casting is used to force a cast from one value to another. While this can be used to avoid type lookups at run-time, it is not recommended for standard use. This is equivalent to a C-style cast.

```
const x: long = 5000000000;
const y = x as! int; // 705032704
```
