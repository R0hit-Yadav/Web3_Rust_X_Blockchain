use crate::{
    order::Order,
    traits::{OrderBook, TradeListener},
};

pub struct MatchingEngine<B: OrderBook, T: TradeListener> {
    pub order_book: B,
    pub trade_listener: T,
}

impl<B: OrderBook, T: TradeListener> MatchingEngine<B, T> {
    pub fn new(order_book: B, trade_listener: T) -> Self {
        Self {
            order_book,
            trade_listener,
        }
    }

    pub fn place_order(&mut self, order: Order) {
        let trades = self.order_book.add_order(order);
        for trade in &trades {
            self.trade_listener.on_trade(&trade);
        }
    }

    pub fn cancel(&mut self, order_id: u64) -> bool {
        self.order_book.cancel_order(order_id)
    }

    pub fn best_bid(&self) -> Option<Order> {
        self.order_book.best_bid()
    }

    pub fn best_ask(&self) -> Option<Order> {
        self.order_book.best_ask()
    }

    pub fn book_depth(&self) -> (Vec<&Order>, Vec<&Order>) {
        self.order_book.full_depth()
    }
}
