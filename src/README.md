# Design Notes:

This is WIP!

Currently it uses `quick-xml` to parse the XML text input file.
It has a simple state machine to pick out the `<title>` tags
within `<book>` tags. We will expand this to pick out other data
we want to extract from book records.

We launch a thread to do the parsing, sending any book data that
is found back to the main thread over a channel. This allows for
cleaner code now, but we will add more features, like a database
connection to store the data and a basic TUI. Having a
multithreaded architecture will be useful for these.

As the project grows we will restructure it. We also plan to try
out different options for allowing the user to interact with the
data. Some ideas are a Golang TUI using Bubbletea, which we think
is nice, or a web API and/or app using some nice current stack.
