import { Client } from "deno_mysql";

type Book = {
  title: string;
  year: number | null;
  isbn: string | null;
  publisher: string | null;
  id: number;
};

type BookData = {
  numPages: number;
  currentPage: number;
  books: Book[];
};

async function num_books(client: Client): Promise<number> {
  const { rows: results } = await client.execute(
    `select count(*) as numRows from book`,
  );

  if (results === undefined) {
    return 0;
  }

  return results[0].numRows;
}

const BOOKS_PER_PAGE: number = 25;

async function run_query(client: Client, page: number): Promise<Book[]> {
  // console.log("Querying collection database.");

  const lowerLimit = BOOKS_PER_PAGE * (page - 1);

  const { rows: books } = await client.execute(
    `select title, year, isbn, publisher, id from book limit ${lowerLimit}, ${BOOKS_PER_PAGE}`,
  );

  return books as Book[];
}

const _testData: Book[] = [
  {
    title: "War and Peace",
    year: 1869,
    isbn: "978-1-85326-062-9",
    publisher: null,
    id: 1,
  },
  {
    title: "The Unbearable Lightness of Being",
    year: 1984,
    isbn: "978-0061148521",
    publisher: "Harper Perennial Modern Classics",
    id: 2,
  },
  {
    title: "Dune",
    year: 1965,
    isbn: "978-0441013593",
    publisher: "Ace",
    id: 3,
  },
];

async function BookData(currentPage: number): Promise<BookData> {
  const client = await new Client().connect({
    hostname: "mariadb",
    username: "mariadb",
    db: "collection",
    password: "p@ssw0rd",
  });

  const numBooks = await num_books(client);
  const numPages = Math.ceil(numBooks / BOOKS_PER_PAGE);

  currentPage = Math.min(currentPage, BOOKS_PER_PAGE);

  const books: Book[] = await run_query(client, currentPage);

  // This is just here as a demonstration of how async works,
  // for now. It would run asyncronously, as the chain of
  // async function calls that got us here is awaited until
  // the promises returned are fulfilled.
  //
  // console.log(books[0])

  return {
    numPages: numPages,
    currentPage: currentPage,
    books: books,
  };
}

export default BookData;
