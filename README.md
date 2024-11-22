# CLZ Book Data App

I use CLZ books to keep track of my paper books collection.
This is an app to read the data in the CLZ library export XML
file and work with it in various ways. It is WIP. This is partly
for my own use, because I like the CLZ mobile app for cataloging
books, and partly an excuse to try out some nice software
development tools.

See the `src` folder [README](src/README.md) for some notes on
the design of the Rust program that reads the CLZ data XML file.

## Database

Notes on our database Docker Compose setup can be found [here](database/README.md).

**To start the database:**

In the project root, assuming you have Docker installed, run

```shell
docker compose up
```

The first time you run this it will create an empty `collection` database.

**Setting up / resetting the database:**

The `db-util` folder has a SQL script to dump all data and reset the
schema, and a little Go program to connect to the database and run it.
See that folder's [README](database/README.md).

## Next

Soon we will start inserting data from the CLZ file into the database
and try out some apps and APIs to view and update the data in various ways.
Some ideas are a web app, maybe with a Deno server, or a Golang TUI
using Bubbletea, which we think is cool.
