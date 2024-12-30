package tea_models

import (
	"fmt"

	// For connecting to local database.
	// TODO: We will add a version using gRPC.
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

// Implement our home model

type HomeModel struct {
	// Available operations
	choices []string
	// Selected operation
	cursor int

	DbConn *dblib.DbConnection

	lastError *error
	statusMsg *string

	importModel *DataImportModel
}

func (m HomeModel) Init() tea.Cmd {
	// No initial command
	return nil
}

func (m HomeModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	case errorMsg:
		m.statusMsg = nil
		m.lastError = msg.err

	case statusMsg:
		m.lastError = nil
		m.statusMsg = msg.msg

	// Handle key presses.
	case tea.KeyMsg:
		switch msg.String() {

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

		// " " is space bar key.
		case "enter", " ":
			return m.handleCommandSelection()
		}
	}

	// Return the updated model to the runtime.
	return m, nil
}

func (m *HomeModel) handleCommandSelection() (tea.Model, tea.Cmd) {
	switch m.choices[m.cursor] {

	// NOTE: The closures below capture our
	// model's current database connection.

	case "Reset Schema":
		return m, func() tea.Msg {
			return resetSchemaCmd(m.DbConn)
		}

	case "Reset Data":
		return m, func() tea.Msg {
			return resetDbCmd(m.DbConn)
		}

	case "Data Import":
		if m.importModel == nil {
			return launchImport(m)
		} else {
			// Continuing an in-process import.
			return m.importModel, nil
		}

	default:
		return m, nil
	}
}

func (m HomeModel) View() string {
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

	// Send string to runtime for rendering.
	return s
}

// Model initialization helper

func InitialModel() HomeModel {
	return HomeModel{
		choices: []string{"Reset Schema", "Reset Data", "Data Import"},
		cursor:  0,
	}
}
