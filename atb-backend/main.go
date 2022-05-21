package main

import (
	"context"
	"fmt"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/jackc/pgx/v4"
	"github.com/jackc/pgx/v4/pgxpool"
)

type DB struct {
	conn *pgxpool.Pool
}

func (db *DB) Query(query string) (pgx.Rows, error) {
	return db.conn.Query(context.Background(), query)
}

func ConnectDB() DB {
	dburl, exists := os.LookupEnv("DBURL")
	if !exists {
		dburl = "postgres://postgres:postgres@localhost:5432/atb"
	}
	conn, err := pgxpool.Connect(context.Background(), dburl)

	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
		os.Exit(1)
	}

	return DB{conn}
}

func main() {

	db := ConnectDB()
	defer db.conn.Close()

	r := gin.Default()
	// r.GET("/accounts", func(c *gin.Context) {

	// 	c.JSON(200, gin.H{
	// 		"message": "pong",
	// 	})
	// })

	r.POST("/signup", func(c *gin.Context) {
		// todo authenticate a server jwt to make sure the request only comes from the plugin
		// i.e. there will be a shared secret between the server plugin and this server
		var json struct {
			UUID  string `json:"uuid" binding:"required"`
			Email string `json:"email" binding:"required"`
		}

		if c.Bind(&json) != nil {
			c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"error": "expected { email, uuid } fields in json"})
		}

		c.Status(http.StatusCreated)
	})
	r.Run() // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}
