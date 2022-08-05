use atb_order_book::{Order, OrderBook, OrderType};

fn main() {
    let mut ob = OrderBook::default();

    let buy = Order {
        id: "1".into(),
        quantity: 100,
        order_type: OrderType::Limit(69),
    };

    ob.submit(buy);

    let buy = Order {
        id: "2".into(),
        quantity: 100,
        order_type: OrderType::Limit(420),
    };

    ob.submit(buy);

    let buy = Order {
        id: "3".into(),
        quantity: 100,
        order_type: OrderType::Market,
    };

    ob.submit(buy);

    let sell = Order {
        id: "4".into(),
        quantity: -100,
        order_type: OrderType::Limit(420),
    };

    ob.submit(sell);

    let sell = Order {
        id: "5".into(),
        quantity: -100,
        order_type: OrderType::Limit(69),
    };

    ob.submit(sell);

    let sell = Order {
        id: "6".into(),
        quantity: -100,
        order_type: OrderType::Market,
    };

    ob.submit(sell);
}
