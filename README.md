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
connection to store the data and a basic TUI. Having a
multithreaded architecture will be useful for these.

As the project grows we will restructure it. We also plan to try
out different options for allowing the user to interact with the
data. Some ideas are a Golang TUI using Bubbletea, which we think
is nice, or a web API and/or app using some nice current stack.

## Database

We have added a Docker Compose service (defined in `compose.yaml`)
with a containerized MariaDB database. It is setup with the following
config parameters:

+ root password `p@ssw0rd`
+ main user `mariadb`
+ main user password `p@ssw0rd`
+ default database `collection`

It has persistent storage when stopped and restarted, by saving the
container's `/var/lib/mysql` directory to the `dbdata` subfolder on 
the host.

If you have the MariaDB client library installed you can access
the containerized database using the command

```shell
mariadb -h localhost -P 3306 -u mariadb -p
```
or using an appropriate connector library for your language.

Soon we will add a utility to interface with the database, to
simplify setting up and resetting the schema. Then we will start
inserting data from the CLZ file into the database and add
interfaces to view and update the data in various ways.
