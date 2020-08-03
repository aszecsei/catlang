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
const a = Alignment::Left;
const val = a as U16;
```

```catlang
enum Alignment: U32 {
  Left, // 0
  Right = 12, // 12
  Center, // 13
}
```
