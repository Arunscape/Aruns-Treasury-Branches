package main

import (
  "fmt"
  "github.com/gorilla/websocket"
  "context"
    "fmt"
    "log"
    "math/rand"
    "os"
    "time"
    "github.com/cockroachdb/cockroach-go/v2/crdb/crdbpgx"
    "github.com/google/uuid"
    "github.com/jackc/pgx/v4"

)

type Order struct {
  price int64
  amount int64
  unix_time int64
  accou
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

func (q BidQueue

func main() {
	fmt.Println("hello")
}
