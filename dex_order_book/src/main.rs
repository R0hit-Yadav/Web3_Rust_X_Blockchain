use dex_order_book::matching_engine::MatchingEngine;
use dex_order_book::order::{Order,OrderSide,OrderType};
use dex_order_book::traits::TradeListener;

struct Logger;

impl TradeListener for Logger {
    fn on_trade(&self, trade: &dex_order_book::trade::Trade) {
        println!("Trade executed: {:?}", trade);
    }
}
fn main() {
    let mut engine = MatchingEngine::new(Logger);

    // Case 1 : match a buy limit with a sell market order 
    let order1 = Order {
        id: 1,
        user_id: 101,
        side: OrderSide::Buy,
        price: 110,
        quantity: 10,
        timestamp: 1,
        order_type: OrderType::Limit,
    };

    let order2 = Order {
        id: 2,
        user_id: 202,
        side: OrderSide::Sell,
        price: 0, 
        quantity: 5,
        timestamp: 2,
        order_type: OrderType::Limit,
    };

    engine.place_order(order1);
    engine.place_order(order2);

    // case 2 : match a but limit with a sell limit order
    let order3 = Order {
        id: 3,
        user_id: 202,
        side: OrderSide::Buy,
        price: 0,
        quantity: 8,
        timestamp: 4,
        order_type: OrderType::Limit,
    };

    let order4 = Order {
        id: 4,
        user_id: 101,
        side: OrderSide::Sell,
        price: 105,
        quantity: 8,
        timestamp: 3,
        order_type: OrderType::Limit,
    };


    engine.place_order(order3);
    engine.place_order(order4);

    // case 3 : match buy price and sell price exactly
    let order5 = Order {
        id: 5,
        user_id: 101,
        side: OrderSide::Buy,
        price: 102,
        quantity: 10,
        timestamp: 5,
        order_type: OrderType::Limit,
    };

    let order6 = Order {
        id: 6,
        user_id: 202,
        side: OrderSide::Sell,
        price: 102,
        quantity: 5,
        timestamp: 6,
        order_type: OrderType::Limit,
    };

    engine.place_order(order5);
    engine.place_order(order6);

    // case 4 : not match anything buy < sell
    let order7 = Order {
        id: 7,
        user_id: 101,
        side: OrderSide::Buy,
        price: 100,
        quantity: 10,
        timestamp: 7,
        order_type: OrderType::Limit,
    };

    let order8 = Order {
        id: 8,
        user_id: 202,
        side: OrderSide::Sell,
        price: 105,
        quantity: 10,
        timestamp: 8,
        order_type: OrderType::Limit,
    };

    engine.place_order(order7);
    engine.place_order(order8);

    // case 5 : check multiple Conditions
    let order9 = Order {
        id: 9,
        user_id: 101,
        side: OrderSide::Buy,
        price: 103,
        quantity: 15,
        timestamp: 9,
        order_type: OrderType::Limit,
    };

    let order10 = Order {
        id: 10,
        user_id: 202,
        side: OrderSide::Sell,
        price: 103,
        quantity: 5,
        timestamp: 10,
        order_type: OrderType::Limit,
    };

    let order11 = Order {
        id: 11,
        user_id: 202,
        side: OrderSide::Sell,
        price: 103,
        quantity: 10,
        timestamp: 11,
        order_type: OrderType::Limit,
    };

    engine.place_order(order9);
    engine.place_order(order10);
    engine.place_order(order11);
}
