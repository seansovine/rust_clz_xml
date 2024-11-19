# Design Notes:

This is WIP!

Currently it uses `quick-xml` to parse the XML text input file.
It has a simple state machine to pick out the `<title>` tags
within `<book>` tags. We will expand this to pick out other data
we want to extract from book records.

We launch a thread to do the parsing and a thread for the database.
These share a channel to the main thread, and there is a separate
channel from the main thread to the database thread. The idea
is that the parser thread is launched to perform one complete task,
while the database thread stands by and runs any tasks that are
sent to it.

Next we will implement the database thread operation. We plan to
use the `sqlx` library for that.

We also plan to try
out different options for allowing the user to interact with the
data. Some ideas are a Golang TUI using Bubbletea, which we think
is nice, or a web API and/or app using some nice current stack.
