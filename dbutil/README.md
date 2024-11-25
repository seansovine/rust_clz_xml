# Database Utility

This folder has a sql script `create_db.sql` to setup or reset our database,
and a simple Go program to connect to our database and execute it. You can
run this simply from this folder with

```shell
make build_run
```

Any updates to the schema should be made in `create_db.sql`.

There is also a script `empty_db.sql` here that can be run to quickly empty
the data from the database, if that's more convenient.

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

We've added the start of a gRPC server to run commands on our database
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

Or from the project root directory

```shell
docker compose up dbutil
```

We will soon flesh out the methods of the service and implement a client
application. Eventually we will have our future TUI use the client methods
to remotely execute commands on the database utility service.
