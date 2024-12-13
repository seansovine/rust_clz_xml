import { Client } from "deno_mysql";

type Book = {
  title: string;
  year: number | null;
  isbn: string | null;
  publisher: string | null;
  id: number;
  // NOTE: We build the string here for now; later
  // we might want to send structured author data.
  authorString: string;
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

  if (results === undefined || results.length == 0) {
    return 0;
  }

  return results[0].numRows;
}

const BOOKS_PER_PAGE: number = 25;

type Author = {
  id: number;
  first_name: string;
  last_name: string;
};

async function book_authors_string(
  client: Client,
  book_id: number,
): Promise<string> {
  const query = `
select a.id
     , a.first_name
     , a.last_name
  from author a
  join author_book ab
    on a.id = ab.author_id
   and ab.book_id = ${book_id}
  `;

  const { rows: authors } = await client.execute(query);
  const authorArray = <Author[]> authors;

  let authorString = "";
  for (let i = 0; i < authorArray.length; i++) {
    const author = authorArray[i];
    // TODO: Better handle case where first or last is null.
    authorString += `${author.last_name}, ${author.first_name}`;
    if (i < authorArray.length - 1) {
      authorString += "\n";
    }
  }

  return authorString;
}

async function run_query(client: Client, page: number): Promise<Book[]> {
  const lowerLimit = BOOKS_PER_PAGE * (page - 1);

  const { rows: books } = await client.execute(
    `select title, year, isbn, publisher, id from book limit ${lowerLimit}, ${BOOKS_PER_PAGE}`,
  );
  const bookArray = <Book[]> books;

  for (const i in bookArray) {
    const bookAsBook = <Book> bookArray[i];
    const authorString: string = await book_authors_string(
      client,
      bookAsBook.id,
    );
    bookAsBook.authorString = authorString;
  }

  return books as Book[];
}

async function BookData(currentPage: number): Promise<BookData> {
  const client = await new Client().connect({
    hostname: "mariadb",
    username: "mariadb",
    db: "collection",
    password: "p@ssw0rd",
  });

  const numBooks = await num_books(client);
  // We will have at least one page (maybe empty).
  const numPages = Math.max(1, Math.ceil(numBooks / BOOKS_PER_PAGE));

  // If the user sends a bad page, we clamp it to the valid range.
  currentPage = Math.max(1, Math.min(currentPage, numPages));

  const books: Book[] = await run_query(client, currentPage);

  // Close this connection since we're done with it.
  await client.close();

  return {
    numPages: numPages,
    currentPage: currentPage,
    books: books,
  };
}

export default BookData;
