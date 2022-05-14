package main

import (
	"fmt"

	"github.com/google/uuid"
	// r "gopkg.in/rethinkdb/rethinkdb-go.v6"
	"github.com/gin-gonic/gin"
)

type User struct {
	id    uuid.UUID
	email string
}

type Account struct {
	id       uuid.UUID
	ownerid  string
	nickname string
}

type Transaction struct {
	id     int64
	fromid uuid.UUID
	toid   uuid.UUID
	amount int64
}

type Order struct {
	price     int64
	amount    int64
	accountid string
}

type BidOrder struct {
	Order
}

type AskOrder struct {
	Order
}

type BidQueue struct {
	list []BidOrder
}

type AskQueue struct {
	list []AskOrder
}

func main() {

	// session, err := r.Connect(r.ConnectOpts{
	// 	Address: "rethinkdb", // endpoint without http
	// })
	// defer session.Close()
	// if err != nil {
	// 	log.Fatalln(err)
	// 	return
	// }

	// infinite loop to handle requests
	// this will be websockets. make a rest api for now
	r := gin.Default()
	r.GET("/create_account", func(c *gin.Context) {
		c.JSON(201, gin.H{
			"message": "created",
		})
	})
	r.Run() // listen and serve on 0.0.0.0:8080

	fmt.Println("hello")
}
