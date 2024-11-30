import { Application } from "jsr:@oak/oak/application";
import { Router } from "jsr:@oak/oak/router";

import routeStaticFilesFrom from "./util/routeStaticFilesFrom.ts";

import BookData from "./bookdata/BookData.ts";

export const app = new Application();
const router = new Router();

router.get("/books", (context) => {
  context.response.type = "application/json";
  context.response.body = BookData();
});

app.use(router.routes());
app.use(routeStaticFilesFrom([
  `${Deno.cwd()}/client/dist`,
  `${Deno.cwd()}/client/public`,
]));

if (import.meta.main) {
  console.log("Server listening on port http://localhost:8000");
  await app.listen({ port: 8000 });
}
