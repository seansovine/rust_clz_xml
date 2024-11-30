## Setup to allow working in the container
## shell with some convenience.

FROM denoland/deno:2.1.1

# Port for Vite
EXPOSE 3000
# Port for server
EXPOSE 8000

# Install useful packages.
RUN apt update && apt install -y sudo vim
# Don't keep old APT data in image.
RUN rm -rf /var/lib/apt/lists/*

# Set a root password for use in container shell.
RUN echo 'root:pw' | chpasswd
RUN echo 'deno:pw' | chpasswd && adduser deno sudo

# For convenience; could get these from a file too.
RUN echo '\nalias c="clear"' >> /etc/bash.bashrc
RUN echo '\nalias ll="ls -Alh"' >> /etc/bash.bashrc

# NOTE: This would be the way for production. But, for
# development it's easier to mount . using a Docker
# Compose volume, so we can edit files and restart
# server without rebuilding the whole Docker image.

# COPY . /app/
# RUN chown -R deno:deno /app

WORKDIR /

# Prefer not to run as root.
USER deno

# We use a start script so we can use a Compose volume.
COPY docker/start.sh .

USER root
RUN chmod a+x start.sh

# NOTE: For development we just run as root, to avoid
# dealing with permissions issues for the Compose
# volume. For production we'd want less-permissioned
# user like deno.

# USER deno

CMD ["/start.sh"]
