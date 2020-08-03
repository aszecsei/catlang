---
id: interfaces
title: Interfaces
sidebar_label: Interfaces
---

Interfaces work much the same as Rust's trait system. Interfaces are a list of required methods, and can be implemented by any struct. Even structs defined externally can have interfaces added (although duplicate interface implementations will throw an error).

```catlang
interface ICollection<T> {

}

impl<T> ICollection<T> for ([]T | [..]T) {

}
```

## Interfaces on Type Unions

Implementing an interface for a type union implements the interface for all types in the union.
