package tea_models

import (
	"context"
	"fmt"
	"sync"

	tea "github.com/charmbracelet/bubbletea"

	"tui/internal/data"
)

// --------------------------
// Bubbletea model definition

type DataImportModel struct {
	homeModel *HomeModel

	ch         *chan any
	cancelFunc *context.CancelFunc

	currentRecord *data.BookRecord
	waiting       bool
}

type initMsg struct {
	msg string
}

func (m DataImportModel) Init() tea.Cmd {
	// This will never get called, because the
	// initial model of the program is a HomeModel.
	return nil
}

func (m DataImportModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	// Update reference on home, because we have a
	// by-value receiver and we return our self.
	m.homeModel.importModel = &m

	switch msg := msg.(type) {

	case initMsg:
		statusMsg := "Parser started."
		m.homeModel.statusMsg = &statusMsg
		return m, nil

	case tea.KeyMsg:
		switch msg.String() {
		case "b":
			// Go back to home menu.
			statusMsg := "Parser running."
			m.homeModel.statusMsg = &statusMsg
			m.homeModel.lastError = nil

			return m.homeModel, nil
		}

	case int: // Debugging case; should *not* get here.
		statusMsg := "Import model received unexpected message."
		m.homeModel.statusMsg = &statusMsg

		return m.homeModel, nil

	case string: // "Done" case.
		// Maybe not necessary, but shouldn't hurt.
		cancel := *m.cancelFunc
		cancel()
		m.cancelFunc = nil

		homeModel := m.homeModel
		homeModel.importModel = nil

		statusMsg := "Parser completed successfully."
		homeModel.statusMsg = &statusMsg
		homeModel.lastError = nil

		return homeModel, nil

	case data.BookRecord:
		m.currentRecord = &msg

		return m, nil
	}

	// If we're waiting and haven't received a book
	// record, user can only go back to home screen.
	if m.waiting == true && m.currentRecord == nil {
		return m, nil
	}

	switch msg := msg.(type) {

	case tea.KeyMsg:

		switch msg.String() {
		case "a":
			if m.currentRecord != nil {
				m.currentRecord = nil
				m.waiting = false

				// TODO: Implement "accept import" using database endpoint.
			}

		case "r":
			if m.currentRecord != nil {
				m.currentRecord = nil
				m.waiting = false
			}

		case "A":
			// TODO: Implement "accept all" using database endpoint.
			// And maybe add confirm, since this could be dangerous!

		case "R":
			cancel := *m.cancelFunc
			cancel()
			m.cancelFunc = nil

			homeModel := m.homeModel
			homeModel.importModel = nil

			statusMsg := "Parsing was cancelled."
			homeModel.statusMsg = &statusMsg
			homeModel.lastError = nil

			return homeModel, nil

			// TODO: Add an "are you sure" state, that stores
			// last message and asks for confirm, then if "yes"
			// re-emits the message. (Think about this.)
		}
	}

	if !m.waiting && m.currentRecord == nil {
		m.waiting = true
		return m, func() tea.Msg {
			ch := *m.ch

			// Blocks until message received.
			// Bubbletea will run this in a goroutine.
			a := <-ch

			switch val := a.(type) {
			case data.BookRecord:
				return val

			case string:
				return "Done"
			}

			return -1
		}
	}

	return m, nil
}

// ---------------
// View functions.

func formatRecord(record data.BookRecord) string {
	recordString := fmt.Sprintf("Title: %s\n", record.Title)

	if record.Year != nil {
		recordString += fmt.Sprintf("Year: %d\n", *record.Year)
	}
	if record.Isbn != nil {
		recordString += fmt.Sprintf("Isbn: %s\n", *record.Isbn)
	}
	if record.Publisher != nil {
		recordString += fmt.Sprintf("Publisher: %s\n", *record.Publisher)
	}

	if len(record.Authors) > 0 {
		recordString += "Authors:\n"
		for _, author := range record.Authors {
			name := fmt.Sprintf("%s, %s", *author.LastName, *author.FirstName)
			recordString += fmt.Sprintf("  > %s\n", name)
		}
	}

	return recordString
}

func (m DataImportModel) View() string {
	s := "Data Import:\n\n"

	if m.currentRecord != nil {
		s += fmt.Sprintf("Found book record:\n\n%s\n", formatRecord(*m.currentRecord))

		s += "  (a) Accept current book for database insert.\n"
		s += "  (r) Reject current book for database insert.\n"
		s += "  (R) To cancel parsing all remaining records.\n\n"

	} else if m.waiting == false {
		s += "Press any key to start receiving.\n\n"
	} else {
		s += "Waiting to receive book.\n\n"
	}

	s += "Press b to return to home.\n"

	return s
}

// -------------------------
// Prototype parser function

// This is here so we can test the TUI
// without connecting to the gRPC endpoint.
//
// TODO: Maybe remove this later, or make
// a test package with this and other test code.

// A failsafe for testing, to make sure this function
// is not run more than once simultaneously.
var runCountMutex sync.Mutex
var runCount = 0

func parser(ch chan<- any) {
	// Increment run count and verify this is only instance.
	runCountMutex.Lock()
	if runCount > 0 {
		panic("Parser goroutine should be a singleton.")
	}
	runCount++
	runCountMutex.Unlock()

	defer close(ch)

	for i := 1; i <= 5; i++ {
		ch <- data.BookRecord{Title: fmt.Sprintf("Book %d", i)}
	}

	ch <- "Done"

	// Decrement run count.
	runCountMutex.Lock()
	runCount--
	runCountMutex.Unlock()
}

// For setting delve breakpoint:
//  b internal/tea-models/data-import-model.go:177
