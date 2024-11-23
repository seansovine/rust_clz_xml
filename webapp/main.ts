// Very basic example of generating an HTML page from
// the results of a query to our collection database.

import { Client } from "https://deno.land/x/mysql/mod.ts";

const client = await new Client().connect({
  hostname: "mariadb",
  username: "mariadb",
  db: "collection",
  password: "p@ssw0rd",
});

const { rows: books } = await client.execute(`select * from book`);

function get_book_item(i) {
	let text: string = "<li>"
	let book = books[i]

	console.log(book)

	text += "book id: " + book.id + "\n"

	text += "<ul>\n"
	text += "  <li>title: " + book.title + "</li>\n"
	text += "  <li>isbn: " + book.isbn + "</li>\n"
	text += "</ul>\n"

	text += "</li>\n"

	return text
}

function handler(_req: Request): Response {
	let response: string = "<html>\n"
	response += "<h1>Books Database</h1>\n"
	response += "<ol>\n"

	for (const i in books) {
		response += get_book_item(i)
	}

	response += "</ol>\n"
	response += "</html>"

	return new Response(response, {
		headers: { "content-type": "text/html; charset=utf-8" },
	});
}

// Starts server on default port 8000.
Deno.serve(handler);
