---
id: project-structure
title: Project Structure
sidebar_label: Project Structure
---

## Imports and Exports

Catlang, by default, does not export anything from a file for use in another file. However, using the `export` keyword before a declaration makes it so that other files are allowed to `import` declarations for later use.

```
// foo.cat
export const x = 12;

// bar.cat
import { x } from "foo";

const fun = () -> {
  print(x);
}
```

Directories can be used to create modules:

```
// module/foo.cat
export const fun = () -> {
  print("Function!");
}

// bar.cat
import { fun } from "module/foo";

const main = () -> {
  fun();
}
```

Imported declarations can be renamed:

```
import { fun as myFun } from "module/foo";

const main = () -> {
  myFun();
}
```

In addition to importing specific declarations, files can import entire files:

```
import * as foo from "module/foo";

const main = () -> {
  foo.fun();
}
```
