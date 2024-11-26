package main

import (
	"log"
	"os"

	dbu "db-util/src/lib"
)

var usage string = "Expects one argument: reset|empty"

func main() {
	args := os.Args

	if len(args) == 1 {
		log.Fatal(usage)
	}

	var err error

	switch args[1] {
	case "reset":
		err = dbu.ResetDb()

	case "empty":
		err = dbu.EmptyDb()

	case "import":
		err = dbu.ImportRecent()

	default:
		log.Fatal(usage)
	}

	if err != nil {
		log.Fatal(err)
	}
}
