package tea_models

import (
	"context"
	"fmt"

	"tui/internal/data"
	"tui/internal/grpc"

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

// Implement our Bubbletea Model

type HomeModel struct {
	choices []string // available operations
	cursor  int      // which item cursor is pointing at

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
			// Closures capture our Model's current DbConn.

			case "Reset Schema":
				return m, func() tea.Msg {
					return resetDbCmd(m.DbConn)
				}

			case "Reset Data":
				return m, func() tea.Msg {
					return resetSchemaCmd(m.DbConn)
				}

			case "Data Import":
				if m.importModel == nil {
					// Launch gRPC parser goroutine.
					ch := make(chan any)
					ctx, cancel := context.WithCancel(context.Background())
					go grpc.Parser(ctx, ch)

					// Try to get first record from parser;
					// if there is none we go back to main menu.
					a := <-ch
					switch val := a.(type) {
					case string:
						statusMsg := "Parse found no records."
						m.statusMsg = &statusMsg
						m.importModel = nil

						cancel()

						return m, nil

					case data.BookRecord:
						i := DataImportModel{homeModel: &m, ch: &ch, cancelFunc: &cancel, currentRecord: &val, waiting: false}
						m.importModel = &i

						return i, nil

					default:
						statusMsg := "An error has occurred."
						m.statusMsg = &statusMsg

						cancel()

						return m, nil
					}
				} else {
					// Allows continuing an in-process import.
					return m.importModel, nil
				}
			}
		}
	}

	// Return the updated Model to the Bubble Tea runtime for processing.
	return m, nil
}

func (m HomeModel) View() string {
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

// Model creation helper

func InitialModel() HomeModel {
	return HomeModel{
		choices: []string{"Reset Schema", "Reset Data", "Data Import"},
		cursor:  0,
	}
}
