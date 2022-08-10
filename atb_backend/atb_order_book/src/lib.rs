use atb_types::prelude::*;
use std::cmp::Ordering;
use std::time::SystemTime;

// negative quantity is selling
// leave possibillity for negative price in case we have a negative oil futures situation
// stop limit, stop orders are processsed by atb_order_queue and submitted here as limit/market orderss
#[derive(Debug, Eq)]
pub struct Order {
    id: String,
    quantity: i128,
    order_type: OrderType,
    time_received: SystemTime,
}
impl Order {
    pub fn new(id: &str, quantity: i128, order_type: OrderType) -> Self {
        let time_received = SystemTime::now();
        let id = id.into();

        Self {
            id,
            quantity,
            order_type,
            time_received,
        }
    }
}
// Market is highest for Buys
// Market is lowest for Sell
// Highest buy at the end of the array
// Lowest sell at the end
impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

#[derive(Debug, Default)]
pub struct OrderBook {
    bid: Vec<Order>,
    ask: Vec<Order>,
}

impl OrderBook {
    pub fn submit(&mut self, order: Order) {
        match order.order_type {
            OrderType::MarketBuy
        }
        if order.is_buy() {
            let index = self.bid.binary_search_by(|o| o.cmp(&order));
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
            (OrderType::Market, OrderType::Market) => other.time_received.cmp(&self.time_received),
            (OrderType::Market, OrderType::Limit(_limit)) => Ordering::Greater,
            (OrderType::Limit(_limit), OrderType::Market) => Ordering::Less,
            (OrderType::Limit(s_limit), OrderType::Limit(o_limit)) => {
                let c = s_limit.cmp(o_limit);
                match s_limit.cmp(o_limit) {
                    Ordering::Greater | Ordering::Less => c,
                    Ordering::Equal => other.time_received.cmp(&self.time_received),
                }
            }
        }
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
