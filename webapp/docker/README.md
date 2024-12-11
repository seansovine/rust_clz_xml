# Docker Containers for Deno App

The Dockerfile `deno.no-volume.dockerfile` copies the app files
into the image on build, so that it can be run without attaching
an external volume. This is less convenient for development, but
more convenient for deployment.

Here are some steps to build and run this container independently.
It assumes the database is running on `localhost:3306` on the host
machine, as can be achieved for example with

```shell
docker compose up -d mariadb
```

in the project root directory.

First build the container image with

```shell
docker remove deno-novolume && \
	docker build -t deno-novolume -f docker/deno.no-volume.dockerfile .
```
This only needs run whenever the Dockerfile has been changed.

Once you've built the container you can stop and remove any running
instances of the container with

```shell
docker stop deno-independent && docker remove deno-independent
```

This is necessary when you want to stop and restart a running container.
Then you can run the container with

```shell
docker run -e "ROLE=server" --name deno-independent -p 8000:8000 --net=host deno-novolume
```
