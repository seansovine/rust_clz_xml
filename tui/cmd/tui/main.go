// CLI for our database utility.
// Based on the first example at:
//  git@github.com:charmbracelet/bubbletea.git

package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	dblib "db-util/src/lib"

	tea "github.com/charmbracelet/bubbletea"
)

// Messages and commands

type errorMsg struct {
	err *error
}

type statusMsg struct {
	msg *string
}

func resetDbCmd(dbConn *dblib.DbConnection) tea.Msg {
	err := dbConn.EmptyDb()
	if err != nil {
		return errorMsg{err: &err}
	}

	status := "Empty database command succeeded."
	return statusMsg{msg: &status}
}

func resetSchemaCmd(dbConn *dblib.DbConnection) tea.Msg {
	err := dbConn.ResetDb()
	if err != nil {
		return errorMsg{err: &err}
	}

	status := "Database schema reset command succeeded."
	return statusMsg{msg: &status}
}

// Implement our Bubbletea model

type model struct {
	choices []string // available operations
	cursor  int      // which item cursor is pointing at

	dbConn    *dblib.DbConnection
	lastError *error

	statusMsg *string
}

func initialModel() model {
	return model{
		choices: []string{"Reset Schema", "Reset Data"},
		cursor:  0,
	}
}

func (m model) Init() tea.Cmd {
	// No initial command
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	case errorMsg:
		m.statusMsg = nil
		m.lastError = msg.err

	case statusMsg:
		m.lastError = nil
		m.statusMsg = msg.msg

	// Handle key presses
	case tea.KeyMsg:

		switch msg.String() {

		// We override ctrl+c?
		case "ctrl+c", "q":
			return m, tea.Quit

		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}

		case "down", "j":
			if m.cursor < len(m.choices)-1 {
				m.cursor++
			}

		// Enter and space bar
		case "enter", " ":
			switch m.choices[m.cursor] {
			// Closures capture our model's current dbConn.

			case "Reset Schema":
				return m, func() tea.Msg {
					return resetDbCmd(m.dbConn)
				}

			case "Reset Data":
				return m, func() tea.Msg {
					return resetSchemaCmd(m.dbConn)
				}
			}
		}
	}

	// Return the updated model to the Bubble Tea runtime for processing.
	return m, nil
}

func (m model) View() string {
	// Build screen text.
	s := "Database management operations:\n\n"

	for i, choice := range m.choices {

		cursor := " "
		if m.cursor == i {
			cursor = ">"
		}

		s += fmt.Sprintf("%s %s\n", cursor, choice)
	}

	s += "\nPress q to quit.\n"

	if m.lastError != nil {
		s += fmt.Sprintf("\n~ Error from last command: %s\n", *m.lastError)
	} else if m.statusMsg != nil {
		s += fmt.Sprintf("\n+ %s\n", *m.statusMsg)
	} else {
		s += "\n\n"
	}

	// Send to the UI for rendering.
	return s
}

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

	model := initialModel()
	model.dbConn = dbConn

	p := tea.NewProgram(model)

	if _, err := p.Run(); err != nil {
		fmt.Printf("A error has occurred running Bubbletea: %v", err)
		os.Exit(1)
	}
}
