use dex_order_book::{
    matching_engine::MatchingEngine,
    order::{Order, OrderSide, OrderType},
    order_book::SimpleOrderBook,
    traits::TradeListener,
};

struct Logger;
impl TradeListener for Logger {
    fn on_trade(&self, trade: &dex_order_book::trade::Trade) {
        println!("Trade Executed: {:?}", trade);
    }
}

fn main() {
    let order_book = SimpleOrderBook::new();
    let mut engine = MatchingEngine::new(order_book, Logger);

    let buy1 = Order {
        id: 1,
        user_id: 101,
        side: OrderSide::Buy,
        price: 120,
        quantity: 20,
        timestamp: 1,
        order_type: OrderType::Market,
    };

    let buy2 = Order {
        id: 2,
        user_id: 102,
        side: OrderSide::Buy,
        price: 120,
        quantity: 10,
        timestamp: 1,
        order_type: OrderType::Market,
    };

    let buy3 = Order {
        id: 3,
        user_id: 103,
        side: OrderSide::Buy,
        price: 40,
        quantity: 10,
        timestamp: 5,
        order_type: OrderType::Market,
    };

    let sell1 = Order {
        id: 11,
        user_id: 201,
        side: OrderSide::Sell,
        price: 100,
        quantity: 10,
        timestamp: 2,
        order_type: OrderType::Market,
    };

    let sell2 = Order {
        id: 12,
        user_id: 202,
        side: OrderSide::Sell,
        price: 100,
        quantity: 10,
        timestamp: 1,
        order_type: OrderType::Market,
    };

    let sell3 = Order {
        id: 13,
        user_id: 203,
        side: OrderSide::Sell,
        price: 300,
        quantity: 10,
        timestamp: 1,
        order_type: OrderType::Market,
    };

    engine.place_order(buy1);
    engine.place_order(buy2);
    engine.place_order(buy3);
    engine.place_order(sell1);
    engine.place_order(sell2);
    engine.place_order(sell3);

    // show full depth
    println!("");
    let (bids, asks) = engine.book_depth();
    println!("Reamining Bids:");
    for b in bids {
        println!("{:?}", b);
    }
    println!("");
    println!("Reamining Asks:");
    for a in asks {
        println!("{:?}", a);
    }
    println!("");
    println!("Best Bid: {:?}", engine.best_bid());
    println!("Best Ask: {:?}", engine.best_ask());
    println!("");

    // cancel order if unmatched
    println!("Cancel Orders:");
    let canceled = engine.cancel(1);
    println!("Is Order 1 Canceled: {}", canceled);
    let canceled = engine.cancel(2);
    println!("Is Order 2 Canceled: {}", canceled);
    let canceled = engine.cancel(3);
    println!("Is Order 3 Canceled: {}", canceled);
    let canceled = engine.cancel(11);
    println!("Is Order 11 Canceled: {}", canceled);
    let canceled = engine.cancel(12);
    println!("Is Order 12 Canceled: {}", canceled);
    let canceled = engine.cancel(13);
    println!("Is Order 13 Canceled: {}", canceled);
}
