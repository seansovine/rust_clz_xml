# CLZ Book Data App

I use CLZ books to keep track of my paper books collection.
This is an app to read the data in the CLZ library export XML
file and work with it in various ways. It is WIP. This is partly
for my own use, because I like the CLZ mobile app for cataloging
books, and partly an excuse to try out some nice software
development tools.

See the `src` folder [README](src/README.md) for some notes on
the evolving design of the Rust program that reads the CLZ data XML
file and inserts the data into a database. Currently we extract
book and author data.

## Database

**To start the database and web app:**

In the project root, assuming you have Docker installed, run

```shell
docker compose up
```

You can add the `--build` flag to force Docker to rebuild the container images,
and add `mariadb` to start only the database service.

The first time you run this it will create an empty `collection` database.

**Setting up / resetting the database:**

The `dbutil` folder has a SQL script to dump all data and reset the
schema, and a little Go program to connect to the database and run it.

See that folder's [README](dbutil/README.md) for instructions on
resetting the database and updating the schema and notes on our
Docker Compose setup.

## Deno Web App

We now have a basic React web app served by Deno to display the book data.
It has a JSON endpoint that serves book record data from our database and
a frontend client that uses that to generate a React Table component displaying
the book information. See [webapp/README](webapp/README.md) for more information.

![screenshot](images/web_app_small.png)

## gRPC Microservice Architecture

We've currently implemented the database
utility as a gRPC microservice that can be called remotely with various
commands from our future TUI. We may also make our Rust XML import program
run as a gRPC service, so it can be run interactively through the TUI.
Then the user can provide input to help avoid adding duplicate data
and for merging data from overlapping records.

The first point of this is to experiment with the technology. But as we expand
the system, we may find more interesting things to do with it. For example
we could run the database service and web app on a Raspberry Pi server so
it's always available.
More details are [here](dbutil/README.md).

## Next

We will continue adding more fields to the book data we extract from
the XML file and insert into the database. We will also keep
working on different ways to view and modify the data.
On idea we're pretty sure about is a Golang TUI using the
[Bubbletea](https://github.com/charmbracelet/bubbletea) framework.
More details and ideas are in the `src` [README](src/README.md).

## Sources

I've gotten help and inspiration from various sources while working
on this. I'll try to cite some of those I drew the most from [here](Credits.md).
