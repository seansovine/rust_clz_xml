import "./App.css";

import BookTable from "./BookTable.tsx";

function App() {
  return (
    <>
      <h1>Book Collection</h1>
      <p className="read-the-docs">
        Information about books in our collection:
      </p>
      <BookTable />
    </>
  );
}

export default App;
