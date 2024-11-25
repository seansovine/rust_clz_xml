# CLZ Book Data App

I use CLZ books to keep track of my paper books collection.
This is an app to read the data in the CLZ library export XML
file and work with it in various ways. It is WIP. This is partly
for my own use, because I like the CLZ mobile app for cataloging
books, and partly an excuse to try out some nice software
development tools.

See the `src` folder [README](src/README.md) for some notes on
the evolving design of the Rust program that reads the CLZ data XML
file and inserts the data into a database.

I've drawn help and inspiration from various sources while working
on this. I'll try to cite some of those I drew the most from [here](Credits.md).

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

## Web App

We've added a Deno service to our Docker Compose setup with a very
basic web server app that generates a simple page from a query to our
`book` database table. More information is [here](webapp/README.md).

## gRPC Microservice Architecture

We're experimenting with this. The current plan is to implement the database
utility first as a gRPC microservice that can be called remotely with various
commands from our future TUI. We can also make our Rust XML import program
run as a microservice. More details are [here](dbutil/README.md).

The point of this is mainly to experiment with the technology. But as we expand
the system, we may find more interesting things to do with it.

## Next

We will continue fleshing out the implementation for extracting different
book data from the XML file and inserting that into the database. We will
try out some apps and APIs to view and update the data in various ways.
Some ideas are a web app, maybe with a Deno server, and a Golang TUI
using Bubbletea, which we think is cool, for managing the data.
More details are in the `src` [README](src/README.md).
