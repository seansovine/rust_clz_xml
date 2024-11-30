# Developer Notes

This is a fork of the repo [react-vite-ts-template](https://github.com/denoland/react-vite-ts-template).
The original `README.md` from that repo is [Here](README_denoland.md).

We have taken the code there and added a Dockerfile and `compose.yaml` to
set up a Docker Compose app to run the Vite development server. Our setup
mounts this repos directory as a Compose volume at `/app` inside the container.
This allows us to edit the React application locally and then Vite will
hot reload it when we save a file with changes. It's great.

## Data Fetching Design

We'll use `fetch` with our JSON endpoint in `main.ts` to get data from our
database to our frontend app.
We may want to consider paging, since showing all of our records in
one table could be overwhelming.

## Using React Table

We found [this](https://tanstack.com/table/v8/docs/framework/react/react-table) site
with a basic usage example. And [here](https://codesandbox.io/p/devbox/zealous-rubin-i3ni9p?file=%2Fsrc%2Findex.css%3A27%2C1)
is a live demo of that code.

To add the React Table dependency we ran in project root:

```shell
deno add npm:@tanstack/react-table
```

## Deno VS Code integration

We installed the Deno VS Code extension.
This is very convenient for Deno development.

We followed the instructions [here](https://docs.deno.com/runtime/getting_started/installation/)
to install Deno locally using `cargo`. The VS Code language
server requires the Deno executable.

On first opening the project in VS Code we used
<kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>p</kbd> to open the
command pallet and ran _Deno: Initialize Workspace Configuration_.
We also ran `deno install` in the local project directory.

## Deno Development in our Docker Container

For only developing the Deno app, you can use the `webapp/Makefile`.
To start the service use

```shell
make dkc_rebuild
```

then if you need a shell inside the container you can use

```shell
make docker_shell
```

The latter is mostly useful for debugging.

The server does a good
job of detecting code changes and reloading the app,
so you shouldn't have to manually restart
it. But for debugging the Deno commands or manually restarting, should you
need these, you can
first modify the `ROLE` environment variable in the `deno` service in
`compose.yaml` to `"manual"` (or any string except "server" or "client").
Then the container will start and wait without running any Deno commands,
so you can start a shell and run things by hand.
