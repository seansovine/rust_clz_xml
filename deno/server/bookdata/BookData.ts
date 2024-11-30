type Book = {
  title: string;
  year: number;
  isbn: string;
  id: number;
};

const testData: Book[] = [
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

function BookData(): Book[] {
  return testData;
}

export default BookData;
