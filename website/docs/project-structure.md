---
id: project-structure
title: Project Structure
sidebar_label: Project Structure
---

Catlang is comprised of scripts and modules declared in different files. A catlang file containing a top-level `import` or `export` is considered a module. A file without any top-level `import` or `export` declarations is treated as a script whose contents are available in the global scope (and therefore to modules as well).

## Exports

### Exporting a Declaration

Any declaration (such as a variable, function, struct, type alias, or interface) can be exported by prefixing the `export` keyword.

```catlang
export const x = 12;
```

### Exporting Statements

Export statements are handy when exports need to be renamed, so the preceding example might be written as:

```catlang
const x = 12;
export { x as myNumber };
```

### Re-Exports

Often modules partially expose the featurs of other modules. A re-export does not import the module locally, or introduce a local variable.

```catlang
export { fun as myFun } from "module/foo";
```

## Imports

Importing is just as easy as exporting from a module. Importing an exported declaration is done via one of these `import` forms:

### Importing a Single Export

```catlang
import { x } from "foo";
```

Imports can also be renamed:

```catlang
import { x as y } from "foo";
```

### Importing a Module

You an import an entire module as a struct-like object:

```catlang
import * as foo from "module/foo";

function main() -> {
  foo::bar();
}
```

### Path Mapping

Deeply-nested folders can lead to very verbose imports. Catlang's build system provides path mapping to let you write shortcuts for these long paths. For example, if there are multiple imports from files in `src/utils/workers/`, you can define a path mapping in the package YAML:

```yaml title="package.yaml"
paths:
  - "@workers": src/utils/workers/
```

```catlang title="src/example.cat"
import { hello } from "@workers/hello.cat"
```

## Importing a Script

Some files should be accessible across the entire project. Manually importing these commonly-used scripts would be a hassle; thus, Catlang provides global "script" files that are imported automatically in every file. These scripts are specified using the `scripts` field in the project YAML; this style of file inclusion should be similar to that of a C-style linker.

Only files that _are_ scripts (that is, they do not contain any top-level imports or exports) are globally imported. Even if your project YAML declares a non-script file as a script, it will not be registered as such unless it matches this criteria.

When using script files, namespaces are recommended to avoid global namespace pollution.

Writing libraries using modules is recommended over using scripts, as they lead to fewer unexpected results (such as accidental namespace collisions).

## Namespaces

Namespaces can be defined using the `namespace` keyword:

```catlang
namespace Shapes {
  export struct Triangle {}
  export struct Circle {}
}

let x = new Shapes::Triangle;
```

It is not recommended to export a single namespace from inside a module, as this leads to unnecessary typing:

```catlang title="src/shapes.cat"
export namespace Shapes {
  export struct Triangle {}
  export struct Circle {}
}
```

```catlang title="src/shapesConsumer.cat"
import * as shapes from "./shapes.cat"
let x = new shapes::Shapes::Triangle; // shapes::Shapes is unnecessary
```

### Aliases

Namespaces and namespace exports can be aliased with the `using` declaration.

```catlang
namespace Shapes {
  export struct Triangle {}
  export struct Circle {}
}

using STriangle = Shapes::Triangle;
```
