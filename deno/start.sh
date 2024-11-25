#!/bin/sh

# Caches dependencies for our app.
deno cache app/main.ts

# Run our server app with network access.
# Watch will detect changes in the app source and reload.
deno run --allow-net --watch app/main.ts
