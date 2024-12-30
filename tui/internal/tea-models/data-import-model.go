package tea_models

import (
	"context"
	"fmt"

	tea "github.com/charmbracelet/bubbletea"

	"tui/internal/data"
	"tui/internal/grpc"
)

// --------------------------
// Bubbletea model definition

type parseData struct {
	recordsFound int
	recordsAdded int
}

type DataImportModel struct {
	homeModel *HomeModel

	parserChan chan any
	cancelFunc context.CancelFunc

	currentRecord *data.BookRecord
	waiting       bool

	parseData
}

func (m DataImportModel) Init() tea.Cmd {
	return nil
}

func (m DataImportModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	// Update reference on home, because we have a
	// by-value receiver and we return our self.
	m.homeModel.importModel = &m

	var counts = func(m DataImportModel) string {
		return fmt.Sprintf("%d books found -- %d books added",
			m.recordsFound,
			m.recordsAdded,
		)
	}

	switch msg := msg.(type) {

	case errorMsg:
		// Maybe not necessary, but shouldn't hurt.
		m.cancelFunc()
		m.cancelFunc = nil

		homeModel := m.homeModel
		homeModel.importModel = nil

		// We forward the command to report the error.
		homeModel.statusMsg = nil
		homeModel.lastError = nil

		return m.homeModel, func() tea.Msg {
			return msg
		}

	case tea.KeyMsg:
		switch msg.String() {

		case "b":
			// Go back to home menu.
			statusMessage := "Parser running."
			m.homeModel.statusMsg = &statusMessage
			m.homeModel.lastError = nil

			return m.homeModel, nil
		}

	case string:
		if msg != "Done" {
			panic("Unexpected message in DataImportModel: " + msg)
		}

		// Maybe not necessary, but shouldn't hurt.
		m.cancelFunc()
		m.cancelFunc = nil

		homeModel := m.homeModel
		homeModel.importModel = nil

		statusMsg := "Parser completed successfully: "
		statusMsg += counts(m)
		homeModel.statusMsg = &statusMsg
		homeModel.lastError = nil

		return homeModel, nil

	case data.BookRecord:
		m.currentRecord = &msg
		m.recordsFound += 1

		return m, nil

	default:
		panic("Unexpected message type in DataImportModel.")
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
				m.recordsAdded += 1
				m.currentRecord = nil
				m.waiting = false

				// TODO: Implement "accept import" using a database endpoint.
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
			m.cancelFunc()
			m.cancelFunc = nil

			homeModel := m.homeModel
			homeModel.importModel = nil

			statusMsg := "Parsing was cancelled: "
			statusMsg += counts(m)
			homeModel.statusMsg = &statusMsg
			homeModel.lastError = nil

			return homeModel, nil

			// TODO: Add an "are you sure" state, that stores
			// last message and asks for confirm, then if "yes"
			// re-emits the message -- or something similar to this.

		default:
			panic("Unexpected message type in DataImportModel.")
		}
	}

	if !m.waiting && m.currentRecord == nil {
		m.waiting = true

		return m, func() tea.Msg {
			return waitForRecord(m.parserChan)
		}
	}

	return m, nil
}

func waitForRecord(ch chan any) tea.Msg {
	// We return this as a tea.Cmd, so
	// Bubbletea will run it in a goroutine.

	// Blocks until message received or channel is closed.
	a, ok := <-ch

	if !ok {
		return "Done"
	}

	switch val := a.(type) {

	case data.BookRecord:
		return val

	case grpc.ParserError:
		return errorMsg{val}

	default:
		panic("Received unexpected type in waitForRecord.")
	}
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
		panic("Import model should have a book or be waiting.")
	} else {
		s += "Waiting to receive book.\n\n"
	}

	s += "Press b to return to home.\n"

	return s
}

// ----------------------
// For use by home model.

func launchImport(m *HomeModel) (tea.Model, tea.Cmd) {
	// Launch gRPC parser goroutine.
	ch := make(chan any)
	ctx, cancel := context.WithCancel(context.Background())
	go grpc.Parser(ctx, ch)

	i := DataImportModel{
		homeModel:     m,
		parserChan:    ch,
		cancelFunc:    cancel,
		currentRecord: nil,
		waiting:       false,
		parseData: parseData{
			recordsFound: 0,
			recordsAdded: 0,
		},
	}

	m.importModel = &i
	i.waiting = true

	return i, func() tea.Msg {
		return waitForRecord(ch)
	}
}
