# catlang

> a painless low-level programming language

[![GitHub Actions](https://github.com/aszecsei/catlang/actions/workflows/test.yml/badge.svg)](https://github.com/aszecsei/catlang/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/aszecsei/catlang/branch/main/graph/badge.svg)](https://codecov.io/gh/aszecsei/catlang)

## Language Guide

The language guide is built using Docusaurus, and can be found [here](https://aszecsei.github.io/catlang).

## Build Tools

This compiler is built using [Rust](https://www.rust-lang.org/en-US/).

### LLVM Integration

This project requires that LLVM 14.0.x be installed on your system. It will search the PATH for the appropriate LLVM installation, but if it cannot find LLVM there you may have to use the `LLVM_SYS_140_PREFIX` environment variable to specify its location.

Specifically, it searches for `llvm-config`, which is _not_ included in Windows release binaries of LLVM. You must compile LLVM from sources on Windows environments:

1. Download the LLVM source files
2. `cd llvm-project-14.0.0`
3. `mkdir build && cd build`
4. `cmake -G "Visual Studio 16 2019" -DCMAKE_BUILD_TYPE="Release" -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;compiler-rt;debuginfo-tests;libc;libclc;libcxx;libcxxabi;libunwind;lld;lldb;openmp;parallel-libs;polly;pstl" -Thost=x64 ..\llvm`
5. `cmake --build . --config Release --target install`

You can then add `llvm-project-14.0.0\build\Release\bin` to your PATH and set it as the `LLVM_SYS_140_PREFIX` environment variable. You should be able to build the project!
