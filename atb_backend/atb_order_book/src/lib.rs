use atb_types::prelude::*;
use std::time::SystemTime;

// negative quantity is selling
// leave possibillity for negative price in case we have a negative oil futures situation
// stop limit, stop orders are processsed by atb_order_queue and submitted here as limit/market orders
// 

#[derive(Debug)]
pub struct BuyOrder {
    id: String,
    type: OrderType,
    account_id: String,
    split_id: u128,
    quantity: u128,
    time_recieved: SystemTime
}

#[derive(Debug)]
pub struct SellOrder {
    id: String,
    type: OrderType,
    account_id: String,
    split_id: u128,
    quantity: u128,
    time_recieved: SystemTime
}   

impl Order {
    pub fn new(id: &str, account_id: &str, quantity: i128, order_type: OrderType) -> Self {
        let time_received = SystemTime::now();
        let id = id.into();
        let split_id = 0;
        let account_id = account_id.into();

        Self {
            id,
            account_id,
            split_id,
            quantity,
            order_type,
            time_received,
        }
    }

    pub fn price(&self) -> i128 {
        match self.order_type {
            OrderType::LimitBuy(price) => self.quantity * price,
            OrderType::LimitSell(price) => self.quantity * price,
            OrderType::MarketBuy | OrderType::MarketSell => todo!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct OrderBook {
    bid: Vec<BuyOrder>,
    ask: Vec<SellOrder>,
}

impl OrderBook {
//    pub fn submit(&mut self, order: Order) {
//        let index = match order.order_type {
//            // go to front of bid queue unless another market order was placed earlier
//            OrderType::MarketBuy => self.bid.partition_point(|other| match other.order_type {
//                OrderType::LimitBuy(_) => true,
//                OrderType::MarketBuy => order.time_received < other.time_received,
//                _ => unreachable!(),
//            }),
//            // go to front of ask queue unlese another market order was placed earlier
//            OrderType::MarketSell => self.ask.partition_point(|other| match other.order_type {
//                OrderType::LimitSell(_) => true,
//                OrderType::MarketSell => order.time_received < other.time_received,
//                _ => unreachable!(),
//            }),
//            OrderType::LimitBuy(price) => {
//                self.bid.partition_point(|other| match other.order_type {
//                    OrderType::MarketBuy => false,
//                    OrderType::LimitBuy(other_price) => other_price < price,
//                    _ => unreachable!(),
//                })
//            }
//            OrderType::LimitSell(price) => {
//                self.ask.partition_point(|other| match other.order_type {
//                    OrderType::MarketSell => false,
//                    OrderType::LimitSell(other_price) => price < other_price,
//                    _ => unreachable!(),
//                })
//            }
//        };
//
//        match order.order_type {
//            OrderType::MarketBuy | OrderType::LimitBuy(_) => self.bid.insert(index, order),
//            OrderType::MarketSell | OrderType::LimitSell(_) => self.ask.insert(index, order),
//        }
//
//        println!("sumbitted an order");
//        dbg!(&self);
//    }
//
//
//    fn order_match(&mut self) -> Option<Transaction> {
//        let bid = self.bid.pop()?;
//        let ask = self.ask.pop()?;
//
//        if let (OrderType::LimitBuy(buy_price), OrderType::LimitSell(sell_price)) =
//            (&bid.order_type, &ask.order_type)
//        {
//            // no matching
//            if buy_price < sell_price {
//                self.bid.push(bid);
//                self.ask.push(ask);
//                return None;
//            }
//
//
//        }
//
//        // orders should be matched and one of them is a market order
//
//        if let OrderType::MarketSell | OrderType::LimitSell(_) = bid.order_type {
//            panic!("no sell orders allowed in bid queue");
//        }
//        if let OrderType::MarketBuy | OrderType::LimitBuy(_) = ask.order_type {
//            panic!("no buy orders allowed in ask queue");
//        }
//
//        if bid.quantity == ask.quantity {
//            // ezpz
//        } else if bid.quantity < ask.quantity {
//            // split ask order
//        } else {
//            // split bid order
//        }
//
//        //        match (bid.order_type, ask.order_type) {
//        //            (OrderType::MarketBuy, OrderType::LimitSell(price)) => {
//        //                if bid.quantity == ask.quantity {
//        //                    // ezpz
//        //                    Some(Transaction::new(
//        //                        &bid.account_id,
//        //                        &ask.account_id,
//        //                        item,
//        //                        bid.quantity,
//        //                        price,
//        //                    ))
//        //                } else if bid.quantity < ask.quantity {
//        //                    // split the ask order
//        //                    self.ask.push(Order {
//        //                        id: ask.id,
//        //                        account_id: ask.account_id,
//        //                        split_id: ask.split_id + 1,
//        //                        quantity: ask.quantity - bid.quantity,
//        //                        order_type: ask.order_type,
//        //                        time_received: ask.time_received,
//        //                    });
//        //
//        //                    Some(Transaction::new(
//        //                        &bid.account_id,
//        //                        &ask.account_id,
//        //                        item,
//        //                        bid.quantity,
//        //                        price,
//        //                    ))
//        //                } else {
//        //                    // split bid order
//        //                    self.bid.push(Order {
//        //                        id: bid.id,
//        //                        account_id: bid.account_id,
//        //                        split_id: bid.split_id + 1,
//        //                        quantity: bid.quantity - ask.quantity,
//        //                        order_type: bid.order_type,
//        //                        time_received: bid.time_received,
//        //                    });
//        //                    Some(Transaction::new(
//        //                        &bid.account_id,
//        //                        &ask.account_id,
//        //                        item,
//        //                        ask.quantity,
//        //                        price,
//        //                    ))
//        //                }
//        //            }
//        //            (OrderType::LimitBuy(buy_price), OrderType::LimitSell(sell_price)) => {
//        //                if buy_price < sell_price {
//        //                    return None;
//        //                }
//        //
//        //                if bid.quantity == ask.quantity {
//        //                    // ezpz
//        //                    Some(Transaction::new(
//        //                        &bid.account_id,
//        //                        &ask.account_id,
//        //                        item,
//        //                        bid.quantity,
//        //                        (buy_price + sell_price) / 2,
//        //                    ))
//        //                } else if bid.quantity < ask.quantity {
//        //                    // split the ask order
//        //                    self.ask.push(Order {
//        //                        id: ask.id,
//        //                        account_id: ask.account_id,
//        //                        split_id: ask.split_id + 1,
//        //                        quantity: ask.quantity - bid.quantity,
//        //                        order_type: ask.order_type,
//        //                        time_received: ask.time_received,
//        //                    });
//        //
//        //                    Some(Transaction::new(
//        //                        &bid.account_id,
//        //                        &ask.account_id,
//        //                        item,
//        //                        bid.quantity,
//        //                        buy_price,
//        //                    ))
//        //                } else {
//        //                    // split bid order
//        //                    self.bid.push(Order {
//        //                        id: bid.id,
//        //                        account_id: bid.account_id,
//        //                        split_id: bid.split_id + 1,
//        //                        quantity: bid.quantity - ask.quantity,
//        //                        order_type: bid.order_type,
//        //                        time_received: bid.time_received,
//        //                    });
//        //                }
//        //            }
//        //            (OrderType::LimitBuy(price), OrderType::MarketSell) => {
//        //                if bid.quantity == ask.quantity {
//        //                    // ezpz
//        //                    Some(Transaction::new(
//        //                        &bid.account_id,
//        //                        &ask.account_id,
//        //                        item,
//        //                        bid.quantity,
//        //                        price,
//        //                    ))
//        //                } else if bid.quantity < ask.quantity {
//        //                    // split the ask order
//        //                    self.ask.push(Order {
//        //                        id: ask.id,
//        //                        account_id: ask.account_id,
//        //                        split_id: ask.split_id + 1,
//        //                        quantity: ask.quantity - bid.quantity,
//        //                        order_type: ask.order_type,
//        //                        time_received: ask.time_received,
//        //                    });
//        //
//        //                    Some(Transaction::new(
//        //                        &bid.account_id,
//        //                        &ask.account_id,
//        //                        item,
//        //                        bid.quantity,
//        //                        price,
//        //                    ))
//        //                } else {
//        //                    // split bid order
//        //                    self.bid.push(Order {
//        //                        id: bid.id,
//        //                        account_id: bid.account_id,
//        //                        split_id: bid.split_id + 1,
//        //                        quantity: bid.quantity - ask.quantity,
//        //                        order_type: bid.order_type,
//        //                        time_received: bid.time_received,
//        //                    });
//        //                }
//        //            }
//        //            _ => unreachable!(),
//        //        }
//    }

    // returns true if successful
    pub fn cancel(&mut self, order: Order) -> Result<(), ()> {
        match order.order_type {
            OrderType::MarketBuy | OrderType::LimitBuy(_) => {
                let index = self.bid.iter().position(|o| o.id == order.id).ok_or(())?;
                self.bid.remove(index);
            }

            OrderType::MarketSell | OrderType::LimitSell(_) => {
                let index = self.ask.iter().position(|o| o.id == order.id).ok_or(())?;
                self.ask.remove(index);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
