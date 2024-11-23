// Very basic example of generating an HTML page from
// the results of a query to our collection database.

import { Client } from "https://deno.land/x/mysql/mod.ts";

function get_book_item(book) {
	let text: string = "<li>"

	text += "book id: " + book.id + "\n"

	text += "<ul>\n"
	text += "  <li>title: " + book.title + "</li>\n"
	text += "  <li>isbn: " + book.isbn + "</li>\n"
	text += "</ul>\n"

	text += "</li>\n"

	return text
}

async function run_query() {
	const client = await new Client().connect({
		hostname: "mariadb",
		username: "mariadb",
		db: "collection",
		password: "p@ssw0rd",
	  });

	  console.log("Querying collection database.")

	  const { rows: books } = await client.execute(`select * from book`);

	  return books
}

async function handler(_req: Request): Promise<Response> {
	let books = await run_query()

	let response: string = "<html>\n"
	response += "<h1>Books Database</h1>\n"
	response += "<ol>\n"

	for (const i in books) {
		response += get_book_item(books[i])
	}

	response += "</ol>\n"
	response += "</html>"

	return new Response(response, {
		headers: { "content-type": "text/html; charset=utf-8" },
	});
}

// Starts server on default port 8000.
Deno.serve(handler);
