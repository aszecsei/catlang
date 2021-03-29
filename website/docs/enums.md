---
id: enums
title: Enums
sidebar_label: Enums
---

## Defining an Enum

```catlang
enum Alignment {
  Left, // 0
  Right, // 1
  Center, // 2
}
const a = Alignment.Left;
const val = a as u16;
```

```catlang
enum Alignment: u32 {
  Left, // 0
  Right = 12, // 12
  Center, // 13
}
```

## Referencing an Enum Variant

When the type of an enum is otherwise known, you can reference a specific variant of that enum without the type:

```catlang
enum Alignment {
  Left, // 0
  Right, // 1
  Center, // 2
}
const a: Alignment = .Left;
```
