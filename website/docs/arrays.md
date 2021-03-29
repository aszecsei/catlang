---
id: arrays
title: Arrays
sidebar_label: Arrays
---

Catlang's arrays differ from C-style arrays in that they contain information about their length (see [C's Biggest Mistake](http://www.drdobbs.com/architecture-and-design/cs-biggest-mistake/228701625)).

```catlang
const finalElement = myArray[myArray.length - 1];
```

Arrays are instantiated using a syntax similar to C, and can be either static- or dynamically-sized:

```catlang
const staticArray = new [25]int;
const dynamicArray = new [..]int;
```

Statically-sized arrays may be created on either the stack or the heap, depending on whether or not the `new` keyword is used. Dynamically-sized arrays _must_ be stored on the heap.

```catlang
const staticArray = [25]int; // stored on the stack
const staticArray2 = new [25]int; // stored on the heap
const dynamicArray = new [..]int; // stored on the heap
const dynamicArray2 = [..]int; // syntax error
```

Arrays of pointers and pointers to arrays are syntactically different:

```catlang
const pointerToArray : *[]int = @myArray;
const arrayOfPointers : []*int = myArray;
```

## Initializing Arrays

Arrays can be initialized when they are created:

```catlang
const growableArray = new [..]int { 0, 1, 2 };
const staticArray = []int { 0, 1, 2 };
```

Note that when initializing a statically-sized array, the array size can be omitted. If the array size is present and does not match the initialized size, an error is thrown.

```catlang
const whoops = [5]int { 0, 1, 2 }; // ERROR: size of initialization does not match array size
```

## Iteration

Arrays can be iterated over using a foreach-style loop (via the [Iterator](iterators) interface):

```catlang
const arr = []int { 1, 2, 3, 4, 5 };
for (x in arr) {
  print(x);
}
```

They can also be iterated over with a C-style for loop:

```catlang
const arr = []int { 1, 2, 3, 4, 5 };
for (i = 0; i < arr.length; ++i) {
  let x = arr[i];
  print(x);
}
```

## Accessing Elements

You can access an element from an array by calling the subscript operator on it with the index of the element you want. Like most languages, indices start at zero.

```catlang
const printFirst = (in: []string) -> {
  print(in[0]);
}
```

## Slices and Ranges

> TODO

## Adding and Changing Elements

Arrays are mutable by default. You can replace any existing element of an array using the subscript operator:

```catlang
const replaceFirst = (replaceIn: []string) -> {
  replaceIn[0] = "new string!";
}
```

You can add, insert, or remove elements from dynamically-sized arrays:

```catlang
let example = new [..]int { 0, 1, 2 };
example.add(4); // { 0, 1, 2, 4 };
example.insert(3, 3); // { 0, 1, 2, 3, 4 };
example.insert(0, -1); // { -1, 0, 1, 2, 3, 4 };
example.removeAt(0); // { 0, 1, 2, 3, 4 };
```
