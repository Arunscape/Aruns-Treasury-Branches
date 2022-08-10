use atb_types::prelude::*;
use std::cmp::Ordering;
use std::time::SystemTime;

// negative quantity is selling
// leave possibillity for negative price in case we have a negative oil futures situation
// stop limit, stop orders are processsed by atb_order_queue and submitted here as limit/market orderss
#[derive(Debug)]
pub struct Order {
    id: String,
    amount: i128,
    order_type: OrderType,
    time_received: SystemTime,
}
impl Order {
    pub fn new(id: &str, amount: i128, order_type: OrderType) -> Self {
        let time_received = SystemTime::now();
        let id = id.into();

        Self {
            id,
            amount,
            order_type,
            time_received,
        }
    }

    pub fn price(&self) -> i128 {
        match self.order_type {
            OrderType::LimitBuy(price) => self.amount * price,
            OrderType::LimitSell(price) => self.amount * price,
            OrderType::MarketBuy | OrderType::MarketSell => todo!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct OrderBook {
    bid: Vec<Order>,
    ask: Vec<Order>,
}

impl OrderBook {
    pub fn submit(&mut self, order: Order) {
        let index = match order.order_type {
            // go to front of bid queue unless another market order was placed earlier
            OrderType::MarketBuy => self.bid.partition_point(|other| match other.order_type {
                OrderType::LimitBuy(_) => true,
                OrderType::MarketBuy => order.time_received < other.time_received,
                _ => unreachable!(),
            }),
            // go to front of ask queue unlese another market order was placed earlier
            OrderType::MarketSell => self.ask.partition_point(|other| match other.order_type {
                OrderType::LimitSell(_) => true,
                OrderType::MarketSell => order.time_received < other.time_received,
                _ => unreachable!(),
            }),
            OrderType::LimitBuy(price) => {
                self.bid.partition_point(|other| match other.order_type {
                    OrderType::MarketBuy => false,
                    OrderType::LimitBuy(other_price) => other_price < price,
                    _ => unreachable!(),
                })
            }
            OrderType::LimitSell(price) => {
                self.ask.partition_point(|other| match other.order_type {
                    OrderType::MarketSell => false,
                    OrderType::LimitSell(other_price) => price < other_price,
                    _ => unreachable!(),
                })
            }
        };

        match order.order_type {
            OrderType::MarketBuy | OrderType::LimitBuy(_) => self.bid.insert(index, order),
            OrderType::MarketSell | OrderType::LimitSell(_) => self.ask.insert(index, order),
        }

        println!("sumbitted an order");
        dbg!(&self);
    }

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
