#!/bin/bash

# Exit on first command not returning success (0).
# Simplifies chained commands.
set -e

echo "Running container startup script."

echo "Role is $ROLE."

inf_loop () {
	# while [ 1 -eq 1 ]; do
	# 	echo "Running infinite loop to keep container alive..."
	# 	sleep 30
	# done

	# Note: A more elegant way to keep the container running.
	tail -f /dev/null
}

if [ "$ROLE" = "server" ]
then
	echo "Starting Deno server."

	# Run Deno commands in app directory.
	cd /app || exit

	# Install the dependencies for our app.
	deno install
	deno run build
	deno run serve

elif [ "$ROLE" = "client" ]
then
	echo "Starting Deno server."

	# Run Deno commands in app directory.
	cd /app || exit

	# Install the dependencies for our app.
	deno install

	# Run our server app with network access.
	# Watch will detect changes in the app source and reload.
	deno run --allow-net dev

else
	echo "Keeping container alive."

	# If we just want a Compose service that runs forever:
	inf_loop
fi

echo "Container startup script complete."
