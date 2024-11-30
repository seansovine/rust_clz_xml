#!/bin/sh

echo "Running container startup script."

inf_loop () {
	# while [ 1 -eq 1 ]; do
	# 	echo "Running infinite loop to keep container alive..."
	# 	sleep 30
	# done

	# Note: A more elegant way to keep the container running.
	tail -f /dev/null
}

# If we just want a Compose service that runs forever:
inf_loop

# Run Deno commands in app directory.
cd /app || exit

# Caches dependencies for our app.
deno install

# Run our server app with network access.
# Watch will detect changes in the app source and reload.
deno run --allow-net dev
