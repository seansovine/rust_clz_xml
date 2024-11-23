FROM denoland/deno:2.1.1

# The default port for Deno's server library.
EXPOSE 8000

WORKDIR /

# Prefer not to run as root.
USER deno

# We use a start script so we can use a Compose volume.
COPY start.sh .

USER root
RUN chmod a+x start.sh

USER deno
CMD ["/start.sh"]
