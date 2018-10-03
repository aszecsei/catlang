[![Built with Mage](https://magefile.org/badge.svg)](https://magefile.org)
[![Go Report Card](https://goreportcard.com/badge/github.com/aszecsei/catlang)](https://goreportcard.com/report/github.com/aszecsei/catlang)
[![CircleCI](https://circleci.com/gh/aszecsei/catlang/tree/master.svg?style=svg)](https://circleci.com/gh/aszecsei/catlang/tree/master)
[![Coverage Status](https://coveralls.io/repos/github/aszecsei/catlang/badge.svg?branch=master)](https://coveralls.io/github/aszecsei/catlang?branch=master)

# Language Guide
The language guide is built using LaTeX, and can be found in the `docs` folder.

# Build Tools
This compiler is built using [Go](https://golang.org/). Package dependencies are managed using [go modules](https://github.com/golang/go/wiki/Modules).

Testing is written using [onpar](https://github.com/apoydence/onpar).

## Magefile
[Mage](https://magefile.org/) is the build tool used for this project, as it is cross-platform (unlike make) and its only dependency is Go, which is required to build this compiler anyway.

To run mage without installing the binary, one can simply use `go run mage.go <target>`.