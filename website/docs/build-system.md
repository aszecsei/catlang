---
id: build-system
title: Build System
sidebar_label: Build System
---

Catlang is designed to scale well from small files to large projects. The CLI can be used to build a single file; Catlang will attempt to locate all files imported by that entry point, and so on, until all required files have been read.

For more complex projects, a configuration file is required, which must be named `package.yaml`.

## Configuration Fields

### name

The name of the project. This, together with the version form an identifier which
is assumed to be completely unique. This name is what other users will refer to
when they import your package. To ensure uniqueness, it is recommended that
names be prefaced with a username - for example, `@aszecsei/my-package`.

- The name may only consist of letters a-z, capital and lowercase; digits; the @ symbol; dashes; and underscores
- The name cannot start with an underscore or dash
- The name must be less than or equal to 256 characters.

### version

The version numbers are assumed to follow the [Semantic Versioning Specification](https://semver.org).

### description

A brief description of your project.

### keywords

An optional list of keywords describing your project.

### homepage

The URL to the project homepage. Example:

```yaml
homepage: https://github.com/owner/project#readme
```

### bugs

### license

### authors

### scripts

### main

### paths

### repository

### dependencies

### devDependencies

### link

A dictionary with two valid keys: `c` and `obj`. These keys should be arrays of paths or a single path.

```yaml
link:
  - c: src/clib/hello.c
  - obj: ["src/lib/world.o", "src/lib/other.o"]
```

Files in the `c` section will be compiled and linked; pre-compiled object files in the `obj` section will simply be linked in the program.

### private

## Example Configuration File

```yaml
name: "@aszecsei/my-library"
version: 0.1.0
description: A basic library to do some basic things.
keywords:
  - catlang
  - example
homepage: https://github.com/aszecsei/catlang#readme
bugs:
  - url: https://github.com/aszecsei/catlang/issues
  - email: aszecsei@gmail.com
license: MIT
authors:
  - name: Alic Szecsei
email: aszecsei@gmail.com
url: https://alic-szecsei.com
scripts:
  - src/scripts/**/*.cat
main: src/main.cat
paths:
  - "@workers": src/utils/workers/
repository: github:aszecsei/catlang
dependencies:
  - "@guyfieri/project": ^2.3.1
  - other: github:username/other
  - yaml: 3.3.x
private: false
```

## Project Initialization

A basic project configuration file can be created using the command `catlang init`. Alternately, passing an argument (`catlang init <name>`) will create a new folder `<name>` in the current directory and create a project configuration file within that new folder.
