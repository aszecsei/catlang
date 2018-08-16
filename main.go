package main

import (
	"fmt"
	"os"
	"time"

	log "github.com/sirupsen/logrus"
	"gopkg.in/alecthomas/kingpin.v2"
)

const (
	// VERSION The CLI version
	VERSION = "0.0.1"
	// AUTHOR The author of the CLI
	AUTHOR = "Alic Szecsei <aszecsei@gmail.com>"
)

var (
	app = kingpin.New("catlang", "Compiler for the CatLang programming language.").Version(VERSION).Author(AUTHOR)

	logLevel = app.Flag("loglevel", "Set the level of logging to show").Default("info").Enum("debug", "info", "warning", "error")

	buildCom      = app.Command("build", "Build an executable.")
	buildOutput   = buildCom.Flag("output", "Output binary name.").Short('o').Default("main").String()
	buildInput    = buildCom.Arg("input", "CatLang source file or package").String()
	buildOptLevel = buildCom.Flag("optimization", "LLVM optimization level").Short('O').Default("0").Int()
)

var startTime time.Time

func main() {
	startTime = time.Now()

	command := kingpin.MustParse(app.Parse(os.Args[1:]))
	switch *logLevel {
	case "debug":
		log.SetLevel(log.DebugLevel)
	case "info":
		log.SetLevel(log.InfoLevel)
	case "warning":
		log.SetLevel(log.WarnLevel)
	case "error":
		log.SetLevel(log.ErrorLevel)
	}

	switch command {
	case buildCom.FullCommand():
		if *buildInput == "" {
			log.Error("No input files passed.")
		}
		printFinishedMessage(startTime, buildCom.FullCommand(), 1)
	}
}

func printFinishedMessage(startTime time.Time, command string, numFiles int) {
	dur := time.Since(startTime)
	fmt.Printf("Finished (%d file(s), %.2fms)\n", numFiles, float32(dur.Nanoseconds()/100000))
}
