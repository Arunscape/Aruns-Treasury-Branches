package main

import (
  "fmt"
  "github.com/gorilla/websocket"
  "github.com/prisma/prisma-client-go"
)

type Order struct {
  price int64
  amount int64
  unix_time int64
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
