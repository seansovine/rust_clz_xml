# Rust CLZ XML Reader

This is a simple program using `quick-xml` to read the 
data from a CLZ book XML export file. 

## Sources:

The use of quick-xml was inspired by

+ [Capnfabs example](https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/)

but mostly based on the simple example from the docs,

+ [Reader docs](https://docs.rs/quick-xml/latest/quick_xml/reader/struct.Reader.html)

## TODO:

- Understand how `read_event_into` works.
- Do some work to start saving the data into structures.
- Start structuring this project in a better way.
- Consider moving this to a separate thread so we can report and control.
- Consider adding some neat TUI. (See Bottom for ideas.)
- Look into connecting to a database to load the data into.
- Make a Golang server that connects to the database and serves some kind of info,
  or a Golang TUI with Bubbletea that allows the users to browse the data.
