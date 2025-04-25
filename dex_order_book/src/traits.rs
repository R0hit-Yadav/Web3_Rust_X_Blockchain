use crate::{order::Order, trade::Trade};

pub trait OrderBook {
    fn add_order(&mut self, order: Order) -> Vec<Trade>;
    fn cancel_order(&mut self, order_id: u64) -> bool;
    fn best_bid(&self) -> Option<Order>;
    fn best_ask(&self) -> Option<Order>;
    fn full_depth(&self) -> (Vec<&Order>, Vec<&Order>);
}

pub trait TradeListener {
    fn on_trade(&self, trade: &Trade);
}
