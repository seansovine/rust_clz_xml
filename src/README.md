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

We have implemented basic database inerting for books using `sqlx`.
We're using `tokio`'s runtime to just block on the async database
methods, but eventually we'll try to take more advantage of the
async nature of `sqlx`.

We also plan to try
out different options for allowing the user to interact with the
data. Some ideas are a Golang TUI using Bubbletea, which we think
is nice, or a web API and/or app using some nice current stack,
maybe based on Deno.

## Error handling

This may be next priority. We will send errors back to the main
thread, where they can be handled and reported.

## Data management

One major issue is disambiguating the data. It's conceivable to
have two books with the same title or two authors with the same
name. I think to start with we will only consider books to be
identical if they have ISBNs recorded and those are both the
same. We will consider authors to be the same if all three names
-- first, middle, and last -- are the same. If any trouble arises
from this we will add an additional disambiguation field to the
`authors` table.

We will create some kind of interface (maybe the Golang TUI) for
merging / separating authors and manually updating data and
associations between data.
