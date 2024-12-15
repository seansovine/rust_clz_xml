package test

import (
	"fmt"
	"sync"
	"tui/internal/data"
)

// -------------------------
// Prototype parser function

// For use in testing the data import option
// without connecting to the gRPC endpoint.

// A failsafe for testing, to make sure this function
// is never running more than once simultaneously.
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
