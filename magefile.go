// +build mage

package main

import (
	"fmt"
	"os"
	"path/filepath"

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
	/*
	if err := sh.RunV("go", "install", "github.com/onsi/ginkgo/ginkgo"); err != nil {
		return err
	}
	gp := os.ExpandEnv("$GOPATH")
	ginkgoPath := filepath.Join(gp, "bin", "ginkgo")
	if err := sh.RunV(ginkgoPath, "-r", "-cover"); err != nil {
		return err
	}
	*/
	if err := sh.RunV("go", "test", "-race", "./..."); err != nil {
		return err
	}
	return nil
}
