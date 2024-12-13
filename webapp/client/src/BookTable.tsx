import { useCallback, useEffect, useState } from "react";

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
  // NOTE: For now author string is built in
  // the backend data endpoint.
  authorString: string;
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
  columnHelper.accessor("authorString", {
    cell: (info) => info.getValue(),
    header: () => <span>Authors</span>,
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

type parentCallbackType = (p: number) => void;

type PageSelectorContext = {
  parentCallback: parentCallbackType;
  initialPage: number;
  totalPages: number;
};

function PageSelector(
  { parentCallback, initialPage, totalPages }: PageSelectorContext,
) {
  const [currentPage, setCurrentPage] = useState(initialPage);

  const pageUpdate = useCallback((n: number) => {
    const newPage: number = Math.max(
      1,
      Math.min(totalPages, n),
    );
    setCurrentPage(newPage);
    parentCallback(newPage);
  });

  const pageChange = useCallback(
    (
      e: React.ChangeEvent<HTMLInputElement>,
    ) => {
      pageUpdate(parseInt(e.target.value));
    },
  );

  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    console.log(e);
    if (e.key == "o") {
      pageUpdate(currentPage - 1);
    } else if (e.key == "p") {
      pageUpdate(currentPage + 1);
    }
  }, [pageUpdate]);

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown]);

  return (
    <>
      <p className="page-selector">
        Page{" "}
        <input
          className="page-selector"
          type="number"
          value={currentPage}
          onChange={(e) => pageChange(e)}
        >
        </input>{" "}
        of {totalPages}:
      </p>
    </>
  );
}

function BookTable() {
  // Arg of useState sets the initial value.
  const [data, setData] = useState(() => []);
  const [currentPage, setCurrentPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);

  useEffect(() => {
    async function apiCall() {
      // Try to fetch JSON data.
      const response = await fetch(
        "/books?" + new URLSearchParams({ page: currentPage }).toString(),
      );
      const bookData: BookData = (await response.json()) as BookData;

      setData(bookData.books);
      setTotalPages(bookData.numPages);

      // NOTE: The PageSelector updates currentPage via a callback.
      // We enforce the same clamping there that is enforced in the
      // JSON endpoint, and don't update currentPage here.
    }

    apiCall();
  }, [currentPage]);

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  // PageSelector will call this when the user
  // changes value of the current page input.
  const pageNumberCallback = useCallback((pageNumber: number) => {
    setCurrentPage(pageNumber);
  }, []);

  return (
    <>
      <PageSelector
        parentCallback={pageNumberCallback}
        initialPage={currentPage}
        totalPages={totalPages}
      >
      </PageSelector>
      <div className="p-2">
        <table>
          <thead>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <th key={header.id} id={header.id}>
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
    </>
  );
}

export default BookTable;
