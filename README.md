# Rust CLZ Data Reader

I use CLZ books to keep track of my paper books collection.
This is an app to read the data in the CLZ library export XML
file and work with it in various ways. It is WIP. It's partly
for my own use, because I like the CLZ mobile app for cataloging
books, and partly an excuse to try out some nice software
development tools.

Currently it uses `quick-xml` to parse the XML text input file.
It has a simple state machine to pick out the `<title>` tags
within `<book>` tags. We will expand this to pick out other data
we want to extract from book records.

It launches a thread to do the parsing, sending the book data
it finds back to the main thread over a channel. This allows for
cleaner code now, but we will add more features, like a database
connection to store the data and a basic TUI.

As the project grows we will restructure it. We also plan to try
out different options for allow the user to interact with the
data. Some ideas are a Golang TUI using Bubbletea, which we think
is nice, or a web API and/or app using some nice current stack.
