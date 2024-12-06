// CLI for our database utility.
//
// Using Bubbletea framework:
//  git@github.com:charmbracelet/bubbletea.git

package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	tea_models "tui/internal/tea-models"

	dblib "db-util/src/lib"

	tea "github.com/charmbracelet/bubbletea"
)

// Helpers for DB connection

func scriptsDir() (string, error) {
	exe, err := os.Executable()
	if err != nil {
		return "", err
	}

	// It is: ../../db-util
	dbUtilPath := filepath.Dir(filepath.Dir(filepath.Dir(exe)))

	return dbUtilPath + "/dbutil/scripts", nil
}

func getDb() (*dblib.DbConnection, error) {
	dbConn, err := dblib.NewDb("localhost")
	if err != nil {
		return nil, err
	}

	scriptsDir, err := scriptsDir()
	if err != nil {
		return nil, err
	}

	dbConn.SetScriptsDir(scriptsDir)

	return dbConn, nil
}

// Main func

func main() {
	dbConn, err := getDb()
	if err != nil {
		log.Fatal("Unable to connect to database.")
	}

	model := tea_models.InitialModel()
	model.DbConn = dbConn

	p := tea.NewProgram(model)

	if _, err := p.Run(); err != nil {
		fmt.Printf("A error has occurred running Bubbletea: %v", err)
		os.Exit(1)
	}
}
