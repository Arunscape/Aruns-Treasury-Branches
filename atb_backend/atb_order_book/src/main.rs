use atb_order_book::Order;
use atb_order_book::OrderBook;
use atb_types::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // key: ticker
    // value: OrderBook
    static ref ORDERBOOKS: HashMap<&'static str, OrderBook> = HashMap::new();
}
fn main() {
    let mut ob = OrderBook::default();

    let buy = Order::new("1", "buyer", 100, OrderType::LimitBuy(69));
    ob.submit(buy);

    let buy = Order::new("2", "buyer", 100, OrderType::LimitBuy(420));
    ob.submit(buy);

    let buy = Order::new("3", "buyer", 100, OrderType::MarketBuy);
    ob.submit(buy);
    let buy = Order::new("4", "buyer", 100, OrderType::MarketBuy);
    ob.submit(buy);

    let buy = Order::new("5", "buyer", 100, OrderType::LimitBuy(1337));
    ob.submit(buy);
    let buy = Order::new("6", "buyer", 100, OrderType::LimitBuy(1337));
    ob.submit(buy);

    let sell = Order::new("01", "seller", 100, OrderType::LimitSell(420));
    ob.submit(sell);

    let sell = Order::new("02", "seller", 100, OrderType::LimitSell(69));
    ob.submit(sell);

    let sell = Order::new("03", "seller", 100, OrderType::MarketSell);
    ob.submit(sell);
    let sell = Order::new("04", "seller", 100, OrderType::MarketSell);
    ob.submit(sell);

    let buy = Order::new("05", "seller", 100, OrderType::LimitSell(42));
    ob.submit(buy);
    let buy = Order::new("06", "seller", 100, OrderType::LimitSell(42));
    ob.submit(buy);
}
