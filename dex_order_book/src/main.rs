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
        price: 80,
        quantity: 10,
        timestamp: 1,
        order_type: OrderType::Limit,
    };

    let buy2 = Order {
        id: 2,
        user_id: 102,
        side: OrderSide::Buy,
        price: 90,
        quantity: 10,
        timestamp: 1,
        order_type: OrderType::Limit,
    };

    let buy3 = Order {
        id: 3,
        user_id: 160,
        side: OrderSide::Buy,
        price: 100,
        quantity: 10,
        timestamp: 1,
        order_type: OrderType::Limit,
    };

    let sell1 = Order {
        id: 11,
        user_id: 201,
        side: OrderSide::Sell,
        price: 100,
        quantity: 10,
        timestamp: 2,
        order_type: OrderType::Limit,
    };

    let sell2 = Order {
        id: 12,
        user_id: 202,
        side: OrderSide::Sell,
        price: 105,
        quantity: 10,
        timestamp: 2,
        order_type: OrderType::Limit,
    };

    let sell3 = Order {
        id: 13,
        user_id: 203,
        side: OrderSide::Sell,
        price: 110,
        quantity: 10,
        timestamp: 2,
        order_type: OrderType::Limit,
    };

    engine.place_order(buy1);
    engine.place_order(buy2);
    engine.place_order(buy3);
    engine.place_order(sell1);
    engine.place_order(sell2);
    engine.place_order(sell3);
    
    // show full depth
    let (bids, asks) = engine.book_depth();
    println!("Bids:");
    for b in bids {
        println!("{:?}", b);
    }
    println!("Asks:");
    for a in asks {
        println!("{:?}", a);
    }
    println!("Best Bid: {:?}", engine.best_bid());
    println!("Best Ask: {:?}", engine.best_ask());

    // cancel order if unmatched
    let canceled = engine.cancel(1);
    println!("Cancel Order 1: {}", canceled);


}
