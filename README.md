# catlang

> a painless low-level programming language

[![CircleCI](https://circleci.com/gh/aszecsei/catlang/tree/master.svg?style=svg)](https://circleci.com/gh/aszecsei/catlang/tree/master)
[![codecov](https://codecov.io/gh/aszecsei/catlang/branch/master/graph/badge.svg)](https://codecov.io/gh/aszecsei/catlang)

## Language Guide

The language guide is built using Docusaurus, and can be found [here](https://aszecsei.github.io/catlang).

## Build Tools

This compiler is built using [Rust](https://www.rust-lang.org/en-US/).

### LLVM Integration

This project requires that LLVM 10.0.x be installed on your system. It will search the PATH for the appropriate LLVM installation, but if it cannot find LLVM there you may have to use the `LLVM_SYS_100_PREFIX` environment variable to specify its location.

Specifically, it searches for `llvm-config`, which is _not_ included in Windows release binaries of LLVM. You must compile LLVM from sources on Windows environments:

1. Download and un-tar the sources from [https://llvm.org/releases/3.9.0/llvm-3.9.0.src.tar.xz](https://llvm.org/releases/3.9.0/llvm-3.9.0.src.tar.xz)
2. `cd llvm-project-10.0.0`
3. `mkdir build && cd build`
4. `cmake -G "Visual Studio 16 2019" -DCMAKE_BUILD_TYPE="Release" -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;compiler-rt;debuginfo-tests;libc;libclc;libcxx;libcxxabi;libunwind;lld;lldb;openmp;parallel-libs;polly;pstl" -Thost=x64 ..\llvm`
5. `cmake --build . --config Release --target install`

You can then add `llvm-project-10.0.0\build\Release\bin` to your PATH and set it as the `LLVM_SYS_100_PREFIX` environment variable. You should be able to build the project!
