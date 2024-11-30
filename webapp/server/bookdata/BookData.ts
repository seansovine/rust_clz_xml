import { Client } from "deno_mysql";

type Book = {
  title: string;
  year: number;
  isbn: string;
  id: number;
};

async function run_query(): Promise<Book[]> {
	const client = await new Client().connect({
		hostname: "mariadb",
		username: "mariadb",
		db: "collection",
		password: "p@ssw0rd",
	  });

	  console.log("Querying collection database.")

	  const { rows: books } = await client.execute(`select title, year, isbn, id from book`);

	  return books as Book[]
}

const _testData: Book[] = [
  {
    title: "War and Peace",
    year: 1869,
    isbn: "978-1-85326-062-9",
    id: 1,
  },
  {
    title: "The Unbearable Lightness of Being",
    year: 1984,
    isbn: "978-0061148521",
    id: 2,
  },
  {
    title: "Dune",
    year: 1965,
    isbn: "978-0441013593",
    id: 3,
  },
];

async function BookData(): Promise<Book[]> {
  const books: Book[] = await run_query()

  // This is just here as a demonstration of how async works,
  // for now. It would run asyncronously, as the chain of
  // async function calls that got us here is awaited until
  // the promises returned are fulfilled.
  //
  // console.log(books[0])

  return books;
}

export default BookData;
