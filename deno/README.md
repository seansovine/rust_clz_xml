# Developer Notes

This is a fork of the repo [react-vite-ts-template](https://github.com/denoland/react-vite-ts-template).
The original `README.md` from that repo is [Here](README_denoland.md).

We have taken the code there and added a Dockerfile and `compose.yaml` to
set up a Docker Compose app to run the Vite development server. Our setup
mounts this repos directory as a Compose volume at `/app` inside the container.
This allows us to edit the React application locally and then Vite will
hot reload it when we save a file with changes. It's great.

## Deno VS Code integration

We installed the Deno VS Code extension, and then followed the
advice [here](https://github.com/denoland/deno/issues/16761) to
use <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>p</kbd> to open the
command pallet and run *Deno: Initialize Workspace Configuration*.
We also ran `deno install` in the project directory. This combination
of things (maybe not all necessary) has allowed us to have excellent
intellisense features in VS Code while working on our Deno project.

We also followed the instructions [here](https://docs.deno.com/runtime/getting_started/installation/)
and chose to install Deno using `cargo`. The VS Code extension's
language server requires that we have the Deno executable installed.

## Data Fetching Design

We'll use `fetch` with a JSON endpoint to get data from our database and
built-in JavaScript methods to serialize and deserialize it. For the
JSON endpoint, we will probably start by adding another route to our
Deno app that connects to the database and serves the data. We could
look at other options, but that should work fine. We may want to consider
paging, however, since showing all of our records in one table could be
overwhelming.

## Using React Table

We found [this](https://tanstack.com/table/v8/docs/framework/react/react-table) site
with a basic usage example. And [here](https://codesandbox.io/p/devbox/zealous-rubin-i3ni9p?file=%2Fsrc%2Findex.css%3A27%2C1)
is a live demo of that code.

To add the React Table dependency we ran in project root:

```shell
deno add npm:@tanstack/react-table
```

We've added a modified version of the example to our app, which displays
a table with information about books in our database. Right now it's just
using a couple hardcoded records. We need to implement our JSON endpoint and
then figure out how in Typescript to deserialize the JSON into objects of
the `Book` type that we've added.

## More Learning

There are language details of Typescript and/or JSX that we need to figure out.
An example is deserializing JSON to objects of a specific type, as just mentioned.
It looks like the answer to that question is
[here](https://basarat.gitbook.io/typescript/type-system/type-assertion#type-assertion-vs-casting),
but it will pay to spend some time understanding some of what's going on here.
