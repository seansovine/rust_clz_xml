package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/go-sql-driver/mysql"
)

func main() {
	fmt.Println("Connecting to database.")

	db := connectDB()
	defer db.Close()

	createTables(db)
}

func connectDB() *sql.DB {
	username := "mariadb"
	password := "p@ssw0rd"
	database := "collection"

	connectionString := fmt.Sprintf("%s:%s@tcp(localhost:3306)/%s", username, password, database)

	db, err := sql.Open("mysql", connectionString)

	if err != nil {
		fmt.Println("Database connection failed.")
		log.Fatal(err)
	}

	return db
}

func createTables(db *sql.DB) {
	_, err := db.Exec("create table if not exists books (id bigint not null)")

	if err != nil {
		fmt.Println("Create table query failed.")
		log.Fatal(err)
	}

	fmt.Println("Executed query to add books table.")
}
