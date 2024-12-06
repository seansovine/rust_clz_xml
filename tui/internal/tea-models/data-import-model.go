package tea_models

import tea "github.com/charmbracelet/bubbletea"

// Bubbletea model definition

type DataImportModel struct {
	homeModel *HomeModel
}

func (m DataImportModel) Init() tea.Cmd {
	// No initial command
	return nil
}

func (m DataImportModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:

		switch msg.String() {
		case "a":
			// TODO: Implement "accept import".

		case "A":
			// TODO: Implement "accept all", and maybe add confirm,
			// since this could be dangerous!

		case "b":
			// Go back to home menu.
			return m.homeModel, nil
		}
	}

	return m, nil
}

func (m DataImportModel) View() string {
	s := "Data Import:\n\n"

	s += "Press b to return to home."

	return s
}
