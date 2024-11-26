# Database Utility

This folder has a sql script `create_db.sql` to setup or reset our database,
and a simple Go console utility to connect to our database and execute it. It also has
code for a gRPC service for querying and upating the database (see more below).

You can run the console utility simply from this folder with

```shell
make build_console
./bin/db-util <reset|empty|import>
```

The `import` option will try to run the `scripts/recent_dump.sql` script in
the database. I use this to import a dump with the recent state of the data.

Any updates to the schema should be made in `create_db.sql`.
There is also a script `empty_db.sql` here that can be run to quickly empty
the data from the database, if that's more convenient. The console utility uses
this for `empty`.

## Notes on our database setup:

We have added a Docker Compose service (defined in `${project_root}/compose.yaml`)
with a containerized MariaDB database. It is set up with the following
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

To only start the database service you can use

```shell
docker compose up mariadb
```

## gRPC Server

We've implemented a simple RPC server to run commands on our database
in Go. It is built and runs inside of a Docker container. We've added it
as a service `dbutil` in our Docker Compose setup, but you can run it
manually with

```shell
make build_docker
make run_docker
```

Or you can debug by running a shell in the container with

```shell
make docker_shell
```

Or from the project root directory you can start the Compose service with

```shell
docker compose up dbutil
```

If you update the gRPC service or protobuf definitions in the `.proto` file,
you need to run

```shell
make proto
```

to regenerate the Go files.

## Next

We'll now implement a basic client to test out the gRPC server.
Eventually we will have our future TUI use the client methods
to remotely execute commands on the database utility service.
