# Design Notes:

This is a parser for the CLZ books XML data export file, written
in Rust. It uses `quick-xml` to extract low-level start, end, and text
XML tags from the input file, and it implements a state machine that is
used to pick out only the data from tags we are interested in.

It launches a thread to do the parsing and a thread for the database
updates.
These share a channel to the main thread, and there is a separate
channel from the main thread to the database thread. The idea
is that the parser thread is launched to perform one complete task,
while the database thread stands by and runs any tasks that are
sent to it.

It uses `sqlx` to insert records into the database for data extracted
by the parser. Since `sqlx` is implemented using async, we effectively
run the database thread as an async task by creating a `tokio` runtime
at the beginning of the thread function and then passing the async
thread main function to into `block_on`. This allows the databse thread
to run as if it were an independent async program that can communicate
with the main thread by message passing.

## Error / Success Handling in the Rust Code

More work needs done in handling errors in the parser and database
update code. Currently it panics on some errors, which is good for
development because it makes them apparent and immediately gives a
backtrace. But for a production system we would want to handle the
errors in a more resilient way and add logging for debugging and
maintentance use. We will continue giving more thought to this as
the system evolves into something more like a final product.

## Data Management

One important issue during data import is disambiguating similar
or overlapping data records that are found. It's conceivable to
have two books with the same title or two authors with the same
name. Further, in some use cases there may be two valid records
of the same book that contain complementary and/or conflicting data,
and we will need a way to identify and merge such records.

We will eventually add a utility to help automate disambiguating
and merging corresponding data records. There are various ways we could do
this, based on matches or similarities in various fields. This is also a
reason for the relatively small number of unique keys in the schema.
