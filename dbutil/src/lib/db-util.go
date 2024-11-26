package lib

import (
	"database/sql"
	"fmt"
	"os"
	"path/filepath"

	_ "github.com/go-sql-driver/mysql"
)

// Database connection params.
var (
	username = "mariadb"
	password = "p@ssw0rd"
	database = "collection"
)

func ResetDb() error {
	_, err := RunSql("create_db.sql")

	return err
}

func EmptyDb() error {
	_, err := RunSql("empty_db.sql")

	return err
}

func ImportRecent() error {
	_, err := RunSql("recent_dump.sql")

	return err
}

func RunSql(sqlFile string) (*sql.Result, error) {
	/// Generic function for running a script
	/// in the dbutil/scripts directory.

	// Can call log.Fatal.
	db, err := connectDB()

	if err != nil {
		return nil, err
	}

	defer db.Close()

	sql, err := readSqlFile(sqlFile)

	if err != nil {
		return nil, err
	}

	result, err := db.Exec(sql)

	if err != nil {
		return nil, err
	}

	return &result, err
}

func connectDB() (*sql.DB, error) {
	// multiStatements lets us execute multiple statements in one query string.
	// We use this since we will execute the entire setup sql script.
	connectionString := fmt.Sprintf("%s:%s@tcp(localhost:3306)/%s?multiStatements=true", username, password, database)

	db, err := sql.Open("mysql", connectionString)

	if err != nil {
		return nil, err
	}

	return db, nil
}

func scriptsDir() (string, error) {
	exe, err := os.Executable()
	if err != nil {
		return "", err
	}

	exePath := filepath.Dir(filepath.Dir(exe))

	return exePath + "/scripts", nil
}

func readSqlFile(scriptFile string) (string, error) {
	scriptPath, err := scriptsDir()
	if err != nil {
		return "", nil
	}

	filename := scriptPath + "/" + scriptFile
	fileBytes, err := os.ReadFile(filename)

	if err != nil {
		return "", err
	}

	return string(fileBytes), nil
}
