#!/bin/sh

deno cache app/main.ts

deno run --allow-net app/main.ts
