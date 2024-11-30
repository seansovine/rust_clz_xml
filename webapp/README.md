# Developer Notes

This app started as a fork of the Deno template in the repo
[react-vite-ts-template](https://github.com/denoland/react-vite-ts-template).
The original README file from that repo is [here](README_denoland.md).
In particular we have kept the nice CSS included in their example
to style our page.

We updated the template to display a table of book information from our
database. To display the data we used a React Table component, and we
added a JSON endpoint to serve the book data. Routing for these endpoints
is handled by the Oak middleware included in the template.

We have  added a Dockerfile and `compose.yaml` to
set up a Docker Compose app to run the Deno and Vite servers. Our setup
mounts this directory as a Compose volume at `/app` inside the container.
This allows us to edit the React application locally and then reload the
updated files inside the container.

## Data Fetching Design

We use `fetch` with a JSON endpoint in `main.ts` to get data from our
database to our frontend app. To handle the async function calls required
we use React's `useEffect` and `useState` hooks to update the table data.

We may add a paging feature, since showing all of our records in
one table is a bit overwhelming when many book records are present.

## Developer Notes

### Using React Table

We found [this](https://tanstack.com/table/v8/docs/framework/react/react-table) site
with a basic usage example. And [here](https://codesandbox.io/p/devbox/zealous-rubin-i3ni9p?file=%2Fsrc%2Findex.css%3A27%2C1)
is a live demo of that code.

To add the React Table dependency we ran in project root:

```shell
deno add npm:@tanstack/react-table
```

### Deno VS Code integration

We installed the Deno VS Code extension.
This is very convenient for Deno development.

We followed the instructions [here](https://docs.deno.com/runtime/getting_started/installation/)
to install Deno locally using `cargo`. The VS Code language
server requires the Deno executable.

On first opening the project in VS Code we used
<kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>p</kbd> to open the
command pallet and ran _Deno: Initialize Workspace Configuration_.
We also ran `deno install` in the local project directory.

### Deno Development in our Docker Container

For only developing the Deno app, you can use the `webapp/Makefile`.
To start the service use

```shell
make dkc_rebuild
```

then if you need a shell inside the container you can use

```shell
make docker_shell
```

For debugging the Deno commands or manually restarting, you can
first modify the `ROLE` environment variable in the `deno` service in
`compose.yaml` to `"manual"` (or any string except "server" or "client").
Then the container will start and wait without running any Deno commands,
so you can start a shell and run things by hand.

__Dev workflows:__

A less efficient workflow is to open a new terminal and use:

```shell
docker compose down deno
docker compose up --build deno
```

This will rebuild and restart the Deno service. Then you can rebuild and
restart the app using <kbd>ctrl</kbd> + <kbd>c</kbd> then

```shell
docker compose up --build deno
```

Docker is smart enough that the state of the container is preserved in
this step as long as the Dockerfile is not changed. This means that the
`deno install` command runs quickly when the container restarts. This
approach is a little clunky, but it works for testing changes in development.

I need to create another service running the Vite version of the client,
which will also rebuild the app when the `.tsx` files are changed. This will
be more efficient.
