package main

import (
	"log"
	"os"

	dbu "db-util/src/lib"
)

var usage string = "Expects one argument: reset|empty|import"

func main() {
	args := os.Args

	if len(args) == 1 {
		log.Fatal(usage)
	}

	var err error

	dbc, err := dbu.NewDb("localhost")

	if err != nil {
		log.Fatal(err)
	}

	switch args[1] {
	case "reset":
		err = dbc.ResetDb()

	case "empty":
		err = dbc.EmptyDb()

	case "import":
		err = dbc.ImportRecent()

	default:
		log.Fatal(usage)
	}

	if err != nil {
		log.Fatal(err)
	}
}
