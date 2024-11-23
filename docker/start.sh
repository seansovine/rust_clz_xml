#!/bin/sh

# Caches dependencies for our app.
deno cache app/main.ts

# Run our server app with network access.
deno run --allow-net app/main.ts
