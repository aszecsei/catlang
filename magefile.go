// +build mage

package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/magefile/mage/mg" // mg contains helpful utility functions, like Deps
	"github.com/magefile/mage/sh"
)

// Default target to run when none is specified
// If not set, running mage will list available targets
// var Default = Build

// A build step that requires additional params, or platform specific steps for example
func Build() error {
	mg.Deps(InstallDeps)
	fmt.Println("Building...")
	return sh.RunV("go", "build", "-o", "catlang.exe", ".")
}

// Manage your deps, or running package managers.
func InstallDeps() error {
	fmt.Println("Installing Deps...")
	return sh.RunV("go", "install")
}

// Clean up after yourself
func Clean() {
	fmt.Println("Cleaning...")
	os.RemoveAll("catlang.exe")
}

func Test() error {
	mg.Deps(InstallDeps)
	fmt.Println("Testing...")

	gocmd := mg.GoCmd()

	if err := sh.RunV(gocmd, "test", "./..."); err != nil {
		return err
	}
	return nil
}

func Coverage() error {
	mg.Deps(Test)
	fmt.Println("Getting coverage...")

	gocmd := mg.GoCmd()

	if err := sh.RunV(gocmd, "test", "./...", "-cover", "-covermode=count", "-coverprofile=out.coverprofile"); err != nil {
		return err
	}

	if err := sh.RunV(gocmd, "get", "github.com/mattn/goveralls"); err != nil {
		return err
	}

	gopath, err := sh.Output(gocmd, "env", "GOPATH")
	if err != nil {
		return fmt.Errorf("can't determine GOPATH: %v", err)
	}
	paths := strings.Split(gopath, string([]rune{os.PathListSeparator}))
	bin := filepath.Join(paths[0], "bin")
	path := filepath.Join(bin, "goveralls")

	if err := sh.RunV(path, "-coverprofile=out.coverprofile", "-service=circle-ci", "-repotoken=$COVERALLS_TOKEN"); err != nil {
		return err
	}

	return nil
}
