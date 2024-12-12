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
  columnHelper.accessor("publisher", {
    header: () => <span>Publisher</span>,
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
      const response = await fetch(
        "/books?" + new URLSearchParams({ page: "1" }).toString(),
      );
      // TODO: Add a component to set the get actual page from user,
      // and use it to set the query parameter here.
      const bookData: BookData = (await response.json()) as BookData;

      // console.log("Fetched book data: ");
      // console.log(bookData[0]);

      setData(bookData.books);
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
