# Design Notes:

This is WIP!

Currently it uses `quick-xml` to parse the XML text input file.
It has a simple state machine to pick out the `<title>` tags
within `<book>` tags. We will soon expand this to pick out other 
data we want to extract from book records, starting with authors.

We launch a thread to do the parsing and a thread for the database.
These share a channel to the main thread, and there is a separate
channel from the main thread to the database thread. The idea
is that the parser thread is launched to perform one complete task,
while the database thread stands by and runs any tasks that are
sent to it.

We've implemented basic database insertion for book records using `sqlx`. 
We're using `tokio`'s runtime to just block on the async database
methods, but eventually we'll try to take more advantage of the
async nature of `sqlx`.

We also plan to keep trying
out different options for interacting with the
data. One favorite idea is a Golang TUI using the Bubbletea framework,
which we think is nice. We've added a very basic web app using
using Deno to construct HTML from a query to the database. We'd like
to explore some of the nice frameworks for building web apps with
Deno.

## Error Handling in the Parser

This may be next priority for the parser. We will send errors back to the main
thread, where they can be handled and reported. Right it now just panics when 
many errors occur -- not good!

## Data Management

One major issue is disambiguating the data. It's conceivable to
have two books with the same title or two authors with the same
name. And even more two records of the same book can have
complementary and/or conflicting data.

At some point we will add a utility to help automate disambiguating
and merging corresponding data. There are various ways we could do
this, based on matches in various fields. But this is also the main
reason for the relative lack of unique keys in the schema.
