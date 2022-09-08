#[allow(dead_code)]
use atb_types::prelude::*;
use std::time::SystemTime;

// negative quantity is selling
// leave possibillity for negative price in case we have a negative oil futures situation
// stop limit, stop orders are processsed by atb_order_queue and submitted here as limit/market orders
//

#[derive(Debug, Clone)]
pub struct BuyOrder {
    id: String,
    account_id: String,
    split_id: usize,
    price: i128,
    quantity: u64,
    time_recieved: SystemTime,
}

impl BuyOrder {
    pub fn new(id: &str, account_id: &str, split_id: usize, price: i128, quantity: u64) -> Self {
        let id = id.into();
        let account_id = account_id.into();
        let time_recieved = SystemTime::now();
        Self {
            id,
            account_id,
            split_id,
            price,
            quantity,
            time_recieved,
        }
    }

    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let x = self.price.cmp(&other.price);
        if let std::cmp::Ordering::Less | std::cmp::Ordering::Greater = x {
            return x;
        }

        let x = self.quantity.cmp(&other.quantity);
        if let std::cmp::Ordering::Less | std::cmp::Ordering::Greater = x {
            return x;
        }

        other.time_recieved.cmp(&self.time_recieved)
    }
}

impl PartialEq for BuyOrder {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price && self.quantity == other.quantity
    }
}

impl PartialOrd for BuyOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SellOrder {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price && self.quantity == other.quantity
    }
}

impl PartialOrd for SellOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct SellOrder {
    id: String,
    account_id: String,
    split_id: usize,
    price: i128,
    quantity: u64,
    time_recieved: SystemTime,
}

impl SellOrder {
    pub fn new(id: &str, account_id: &str, split_id: usize, price: i128, quantity: u64) -> Self {
        let id = id.into();
        let account_id = account_id.into();
        let time_recieved = SystemTime::now();
        Self {
            id,
            account_id,
            split_id,
            price,
            quantity,
            time_recieved,
        }
    }
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let x = other.price.cmp(&self.price);
        if let std::cmp::Ordering::Less | std::cmp::Ordering::Greater = x {
            return x;
        }

        let x = self.quantity.cmp(&other.quantity);
        if let std::cmp::Ordering::Less | std::cmp::Ordering::Greater = x {
            return x;
        }

        other.time_recieved.cmp(&self.time_recieved)
    }
}

#[derive(Debug, Default)]
pub struct OrderBook {
    bid: Vec<BuyOrder>,
    ask: Vec<SellOrder>,
}

impl OrderBook {
    /// ```
    /// use atb_types::prelude::*;
    /// use atb_order_book::{OrderBook, BuyOrder};
    ///
    /// let mut order_book = OrderBook::default();
    ///
    /// // higher price gets priority
    /// let buy_order = BuyOrder::new("0", "buyer", 0, 10, 100);
    /// order_book.submit_buy(buy_order);
    ///
    /// let buy_order = BuyOrder::new("1", "buyer", 0, 100, 100);
    /// order_book.submit_buy(buy_order);
    ///
    /// assert_eq!(order_book.buy_orders_ids(), vec!["0", "1"]);
    ///
    /// // quantity gets next priority
    /// let buy_order = BuyOrder::new("2", "buyer", 0, 100, 1000);
    /// order_book.submit_buy(buy_order);
    /// assert_eq!(order_book.buy_orders_ids(), vec!["0", "1", "2"]);
    ///
    /// // earlier order gets priority  if price and quantity are the same
    /// let buy_order = BuyOrder::new("3", "buyer", 0, 100, 1000);
    /// order_book.submit_buy(buy_order);
    /// assert_eq!(order_book.buy_orders_ids(), vec!["0", "1", "3", "2"]);
    ///
    /// ```
    pub fn submit_buy(&mut self, order: BuyOrder) {
        let index = self.bid.partition_point(|x| x < &order);
        self.bid.insert(index, order);
    }

    /// ```
    /// use atb_types::prelude::*;
    /// use atb_order_book::{OrderBook, SellOrder};
    ///
    /// let mut order_book = OrderBook::default();
    ///
    /// // lower price gets priority
    /// let sell_order = SellOrder::new("0", "seller", 0, 100, 100);
    /// order_book.submit_sell(sell_order);
    ///
    /// let sell_order = SellOrder::new("1", "seller", 0, 10, 100);
    /// order_book.submit_sell(sell_order);
    ///
    /// assert_eq!(order_book.sell_orders_ids(), vec!["0", "1"]);
    /// dbg!(&order_book);
    ///
    /// // quantity gets next priority
    /// let sell_order = SellOrder::new("2", "seller", 0, 10, 1000);
    /// order_book.submit_sell(sell_order);
    /// assert_eq!(order_book.sell_orders_ids(), vec!["0", "1", "2"]);
    ///
    /// // earlier order gets priority  if price and quantity are the same
    /// let sell_order = SellOrder::new("3", "seller", 0, 10, 1000);
    /// order_book.submit_sell(sell_order);
    /// assert_eq!(order_book.sell_orders_ids(), vec!["0", "1", "3", "2"]);
    ///
    /// ```
    pub fn submit_sell(&mut self, order: SellOrder) {
        let index = self.ask.partition_point(|x| x < &order);
        self.ask.insert(index, order);
    }

    pub fn buy_orders_ids(&self) -> Vec<String> {
        self.bid.iter().map(|x| x.id.clone()).collect()
    }

    pub fn sell_orders_ids(&self) -> Vec<String> {
        self.ask.iter().map(|x| x.id.clone()).collect()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn buy_q_sorted() {
//     }
// }
