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

// Database connection type.

type DbConnection struct {
	db *sql.DB
}

func NewDb(host string) (*DbConnection, error) {
	db, err := connectDB(host)

	if err != nil {
		return nil, err
	}

	return &DbConnection{db: db}, nil
}

func (dbc *DbConnection) Close() {
	dbc.db.Close()
}

func (dbc *DbConnection) ResetDb() error {
	_, err := runSql(dbc.db, "create_db.sql")

	return err
}

func (dbc *DbConnection) EmptyDb() error {
	_, err := runSql(dbc.db, "empty_db.sql")

	return err
}

func (dbc *DbConnection) ImportRecent() error {
	_, err := runSql(dbc.db, "recent_dump.sql")

	return err
}

func connectDB(host string) (*sql.DB, error) {
	// multiStatements lets us execute multiple statements in one query string.
	// We use this since we will execute the entire setup sql script.
	connectionString := fmt.Sprintf("%s:%s@tcp(%s:3306)/%s?multiStatements=true", username, password, host, database)

	db, err := sql.Open("mysql", connectionString)

	if err != nil {
		return nil, err
	}

	return db, nil
}

// Helpers for running our SQL scripts.

func runSql(db *sql.DB, sqlFile string) (*sql.Result, error) {
	/// Generic function for running a script
	/// in the dbutil/scripts directory.

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
