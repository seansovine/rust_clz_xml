package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	dblib "db-util/src/lib"

	tea "github.com/charmbracelet/bubbletea"
)

type model struct {
	choices []string // items on the to-do list
	cursor  int      // which to-do list item our cursor is pointing at

	dbConn    *dblib.DbConnection
	lastError *error

	statusMsg *string
}

func initialModel() model {
	return model{
		// Our to-do list is a grocery list
		choices: []string{"Reset Schema", "Reset Data"},
		cursor:  0,
	}
}

func (m model) Init() tea.Cmd {
	// Just return `nil`, which means "no I/O right now, please."
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	// Is it a key press?
	case tea.KeyMsg:

		// Cool, what was the actual key pressed?
		switch msg.String() {

		// These keys should exit the program.
		case "ctrl+c", "q":
			return m, tea.Quit

		// The "up" and "k" keys move the cursor up
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}

		// The "down" and "j" keys move the cursor down
		case "down", "j":
			if m.cursor < len(m.choices)-1 {
				m.cursor++
			}

		// The "enter" key and the spacebar (a literal space) toggle
		// the selected state for the item that the cursor is pointing at.
		case "enter", " ":
			var err error = nil
			switch m.choices[m.cursor] {
			case "Reset Schema":
				err = m.dbConn.ResetDb()

			case "Reset Data":
				err = m.dbConn.EmptyDb()
			}

			if err != nil {
				m.statusMsg = nil
				m.lastError = &err
			} else {
				m.lastError = nil
				statusMsg := "Command succeeded."
				m.statusMsg = &statusMsg
			}
		}
	}

	// Return the updated model to the Bubble Tea runtime for processing.
	// Note that we're not returning a command.
	return m, nil
}

func (m model) View() string {
	// The header
	s := "Database management operations:\n\n"

	// Iterate over our choices
	for i, choice := range m.choices {

		// Is the cursor pointing at this choice?
		cursor := " " // no cursor
		if m.cursor == i {
			cursor = ">" // cursor!
		}

		// Render the row
		s += fmt.Sprintf("%s %s\n", cursor, choice)
	}

	// The footer
	s += "\nPress q to quit.\n"

	if m.lastError != nil {
		s += fmt.Sprintf("\n~ Error from last command: %s\n", *m.lastError)
	} else if m.statusMsg != nil {
		s += fmt.Sprintf("\n+ %s\n", *m.statusMsg)
	} else {
		s += "\n\n"
	}

	// Send the UI for rendering
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
		fmt.Printf("A error has ocurred running Bubbletea: %v", err)
		os.Exit(1)
	}
}
