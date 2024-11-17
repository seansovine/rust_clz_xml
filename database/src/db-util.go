package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	_ "github.com/go-sql-driver/mysql"
)

func main() {
	fmt.Println("Connecting to database.")

	// Can call log.Fatal.
	db := connectDB()
	defer db.Close()

	err := createTables(db)

	if err != nil {
		log.Fatal(err)
	}
}

func connectDB() *sql.DB {
	username := "mariadb"
	password := "p@ssw0rd"
	database := "collection"

	// multiStatements lets us execute multiple statements in one query string.
	// We use this since we will execute the entire setup sql script.
	connectionString := fmt.Sprintf("%s:%s@tcp(localhost:3306)/%s?multiStatements=true", username, password, database)

	db, err := sql.Open("mysql", connectionString)

	if err != nil {
		fmt.Println("Database connection failed.")

		log.Fatal(err)
	}

	return db
}

func createTables(db *sql.DB) error {
	// testSql := `create table if not exists book (id int not null auto_increment, primary key (id));
	//             create table if not exists author (id int not null, primary key (id));`

	fileSql, err := readSqlFile()

	if err != nil {
		fmt.Println("Create table query failed.")

		return err
	}

	_, err = db.Exec(fileSql)

	if err != nil {
		fmt.Println("Create table query failed.")

		return err
	}

	fmt.Println("Executed table setup query.")

	return nil
}

func readSqlFile() (string, error) {
	filename := "create_db.sql"
	fileBytes, err := os.ReadFile(filename)

	if err != nil {
		return "", err
	}

	return string(fileBytes), nil
}
