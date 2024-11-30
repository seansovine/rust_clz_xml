import { useEffect, useState } from "react";

import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";

import "./BookTable.css";

type Book = {
  title: string;
  year: number;
  isbn: string;
  id: number;
};

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
];

const newBook = {
  title: "Dune",
  year: 1965,
  isbn: "978-0441013593",
  id: 3,
};

// Testing type assertion, which we can
// use when deserializing the data JSON.
_testData.push(newBook as Book);

const columnHelper = createColumnHelper<Book>();

const columns = [
  columnHelper.accessor("title", {
    cell: (info) => info.getValue(),
    header: () => <span>Title</span>,
    footer: (info) => info.column.id,
  }),
  columnHelper.accessor("year", {
    header: () => <span>Year</span>,
    cell: (info) => info.renderValue(),
    footer: (info) => info.column.id,
  }),
  columnHelper.accessor("isbn", {
    header: () => <span>ISBN</span>,
    cell: (info) => info.renderValue(),
    footer: (info) => info.column.id,
  }),
  columnHelper.accessor("id", {
    header: () => <span>book-id</span>,
    cell: (info) => info.renderValue(),
    footer: (info) => info.column.id,
  }),
];

function BookTable() {
  // Arg of useState sets the initial value.
  const [data, setData] = useState(() => []);

  useEffect(() => {
    async function apiCall() {
      // Try to fetch JSON data.
      const response = await fetch("/books");
      const bookData: Book[] = (await response.json()) as Book[];

      // console.log("Fetched book data: ");
      // console.log(bookData[0]);

      setData(bookData);
    }

    apiCall();
  }, []);

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(), //row model
  });

  return (
    <div className="p-2">
      <table>
        <thead>
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header) => (
                <th key={header.id}>
                  {header.isPlaceholder ? null : flexRender(
                    header.column.columnDef.header,
                    header.getContext(),
                  )}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.map((row) => (
            <tr key={row.id}>
              {row.getVisibleCells().map((cell) => (
                <td key={cell.id}>
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default BookTable;
