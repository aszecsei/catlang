---
id: casting
title: Casting
sidebar_label: Casting
---

Casting is a way to check the type of an instance or to treat an instance of one type as another.

## Type Checking

Type checking is performed with the `is` operator:

```catlang
const x: int | string = otherFunction();
if (x is int) {
  print("x is an int!");
} else {
  print("x is a string!");
}
```

## Safe Casting

Safe casting checks if a casting operator exists between the two provided types. If not enough information exists to ensure that such a casting operator exists (for example, if the provided variable's type is a type union) then it will return an optional type, equivalent to `type | null`; this safe cast must be evaluated at run-time. If run-time type information indicates that the variable's _actual value_ is of the casted type, the cast will succeed. Otherwise, the cast will return `null`.

However, if a cast operator exists between the two types, then the cast operator will be invoked and the resultant type will simply be the casted-to type.

```catlang
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

```catlang
if (let castX = x as int) {
  myFunc(castX); // OK
}
```

This has the twofold benefit of being more concise and scoping the `castX` variable to the `if` block.

Internally, numbers can be safely cast to any number type that is guaranteed not to lose data. For example:

```catlang
const ch: char = 'a';
const xInt: int = ch as int; // OK!
const yInt: int = -2;
const yChar = yInt as char; // ERROR!
```

## Unsafe Casting

Unsafe casting is used to force a cast from one value to another. While this can be used to avoid type lookups at run-time, it is not recommended for standard use. This is equivalent to a C-style cast.

```catlang
const x: long = 5000000000;
const y = x as! int; // 705032704
```

## Type Coercion

A limited amount of type coercion is permitted - namely, when it is completely unambiguous how to get from one type to another, and the transformation is guaranteed to be safe.

### Stricter Qualification

Variables in catlang can be coerced to constant versions of those variables:

```catlang
let x: int = 32;
const y: int = x;
```

Non-volatile variables can also be coerced to volatile variables:

```catlang
let x: int = 32;
let y: volatile int = x;
```

### Integer and Float Widening

Ints can coerce to ints which can represent every value of the old type, and likewise floats coerce to floats which can represent any value of the old type.

```catlang
let a: u8 = 250;
let b: u16 = a;
let c: u32 = b;
let d: u64 = d;
```

```catlang
let a: u8 = 250;
let b: i16 = a;
```

```catlang
let a: float = 12.34;
let b: double = a;
```

### Optionals

Non-optional types can be coerced to their optional equivalents.

```catlang
let a: int = 16;
let b: int? = a;
```

### Union Types

Types can be coerced to any union type that contains the type.

```catlang
let a: int = 16;
let b: int | bool = a;
```

### Any

The `any` type can be coerced into any other type.

```catlang
let a: any = 16;
let b: int = a;
```

This can be very dangerous, as there is no guarantee that re-interpreting an `any` value will not lead to undefined behavior.
