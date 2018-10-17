---
id: generics
title: Generics
sidebar_label: Generics
---

Generics look similar to generics in other languages:

```
const contains = <T>(arr: []T, value: T) -> {
  for (x in arr) {
    if (x == value) {
      return true;
    }
  }
  return false;
}
```

When generic functions are called, the generic type can be inferred:

```
const arr = []int { 1, 2, 3, 4, 5 };
const result = contains(arr, 4);
```

However, sometimes the generic type cannot be inferred and must be made explicit:

```
const arrayFactory = <T>(count: number) {
  return new [count]T;
}
const arr = arrayFactory(10); // ERROR!
const arr = arrayFactory<int>(10); // OK!
```

Structs can also be made generic:

```
struct Node<K, V> {
  key: K;
  value: V;
  leftChild?: Node<K, V>;
  rightChild?: Node<K, V>;
}
struct Tree<K, V> {
  root: Node<K, V>;
}

Tree<K, V>::get = (this, key: K) -> {
  const getHelper = (node?: Node<K, V>) -> {
    if (node) {
      if (node.key == key) {
        return node.value;
      } else {
        if (node.key < key) {
          return getHelper(node.rightChild);
        } else {
          return getHelper(node.leftChild);
        }
      }
    } else {
      return null;
    }
  }
  return getHelper(this.root);
}

const t: Tree<int, string> = treeMaker();
if (let v = t.get(12)) {
  print(v);
}
```
