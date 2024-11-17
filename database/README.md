# Database Utility

This folder has a sql script `create_db.sql` to setup or reset our database,
and a simple Go program to connect to our database and execute it. You can
run this simply from this folder with

```shell
make build_run
```

Any updates to the schema should be made in `create_db.sql`.

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
