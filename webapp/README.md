# Developer Notes

This is a basic React web app for displaying information about our book
records, hosted by a containerized Deno server.
It started as a fork of the Deno template in the repo
[react-vite-ts-template](https://github.com/denoland/react-vite-ts-template).
The original README file for that repo is [here](README_denoland.md), and
describes its features in more detail.
Note that we have kept the nice CSS styling included the template for use
in our app.

Our app displays a table of book information from our
database, using a component from the React Table library. We have a
a JSON endpoint to serve the book data, which is called by the frontend client
to populate its table. Routing for these endpoints
is handled by the Oak middleware that is included in the template.

We have added a Dockerfile to containerize this app and a service in [/compose.yaml](../compose.yaml)
to run the Deno and Vite servers. Our Compose setup
mounts this directory as a volume at `/app` inside the container.
This approach allows us to edit the React application locally and then reload the
updated files inside the container, without rebuilding the container.

## Data Fetching Design

We use a `fetch` call in the client to call a JSON endpoint in `main.ts` to
get data from our database to the frontend app. To handle the async `fetch`-related
function calls while updating the table data, we use React's `useEffect` and `useState` hooks.

We may add a paging feature to the JSON endpoint, since showing all of our records in
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
server requires the Deno executable. (Recall that our actual Deno server
runs in a Docker container.)

On first opening the project in VS Code we used
<kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>p</kbd> to open the
command pallet and ran _Deno: Initialize Workspace Configuration_.
We also ran `deno install` in the local project directory to install
dependencies for use by the VS Code extension.

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
and you can run a shell in the container to run the Deno commands by hand.

__Dev workflows:__

A less efficient workflow is to open a new terminal and use:

```shell
docker compose down deno
docker compose up --build deno
```

This will rebuild and restart the Deno service. Then if you need to rebuild and
restart the app you can first stop it using <kbd>ctrl</kbd> + <kbd>c</kbd> then
restart it using

```shell
docker compose up --build deno
```

Docker is smart enough that the state of the container is preserved in
this step as long as the Dockerfile is not changed. This means that the
`deno install` command runs quickly when the container restarts. This
approach is a little clunky, but it works for testing changes in development.

I need to create another service using the Vite to run the client.
Vite will rebuild the React app whenever the `.tsx` files are changed, providing
a more efficient development workflow.
