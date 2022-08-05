use lazy_static::lazy_static;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, PartialEq)]
pub enum OrderType {
    Market,
    Limit(i128),
}

// negative quantity is selling
// leave possibillity for negative price in case we have a negative oil futures situation
// stop limit, stop orders are processsed by atb_order_queue and submitted here as limit/market orderss
#[derive(Debug, Eq)]
pub struct Order {
    pub id: String,
    pub quantity: i128,
    pub order_type: OrderType,
}

// Market is highest for Buys
// Market is lowest for Sell
impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        // quantity must be the same sign else bad things happen
        // only works on int types
        if self.quantity ^ other.quantity < 0 {
            panic!("tried to compare a buy order with a sell order. Can only compare buy<=>buy or sell<=>sell")
        }
        self.cmp(other)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.quantity ^ other.quantity < 0 {
            return None;
        }

        Some(self.cmp(other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        if self.quantity ^ other.quantity < 0 {
            panic!("tried to compare a buy order with a sell order. Can only compare buy<=>buy or sell<=>sell")
        }
        match (&self.order_type, &other.order_type) {
            (OrderType::Market, OrderType::Market) => true,
            (OrderType::Market, OrderType::Limit(_limit)) => false,
            (OrderType::Limit(_limit), OrderType::Market) => false,
            (OrderType::Limit(s_limit), OrderType::Limit(o_limit)) => s_limit == o_limit,
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
        // pick matching algorithm
        if order.is_buy() {
            let index = self.bid.binary_search_by(|o| order.cmp(o));
            let index = match index {
                Ok(idx) => idx,
                Err(idx) => idx,
            };
            self.bid.insert(index, order)
        } else {
            let index = self.ask.binary_search_by(|o| o.cmp(&order));
            let index = match index {
                Ok(idx) => idx,
                Err(idx) => idx,
            };
            self.ask.insert(index, order)
        }

        // TODO: match some orders togeether
        println!("sumbitted an order");
        dbg!(&self);
    }

    // returns true if successful
    pub fn cancel(&mut self, order: Order) -> bool {
        if order.is_buy() {
            if let Some(index) = self.bid.iter().position(|o| o.id == order.id) {
                self.bid.remove(index);
                return true;
            }
        } else {
            if let Some(index) = self.ask.iter().position(|o| o.id == order.id) {
                self.ask.remove(index);
                return true;
            }
        }
        false
    }
}

impl Order {
    pub fn is_buy(&self) -> bool {
        self.quantity > 0
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.order_type, &other.order_type) {
            (OrderType::Market, OrderType::Market) => Ordering::Equal,
            (OrderType::Market, OrderType::Limit(_limit)) => {
                if self.is_buy() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (OrderType::Limit(_limit), OrderType::Market) => {
                if other.is_buy() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (OrderType::Limit(s_limit), OrderType::Limit(o_limit)) => s_limit.cmp(&o_limit),
        }
    }
}

lazy_static! {
    // key: ticker
    // value: OrderBook
    static ref ORDERBOOKS: HashMap<&'static str, OrderBook> = HashMap::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
