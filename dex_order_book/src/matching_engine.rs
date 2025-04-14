use crate::{order::Order,order_book::SimpleOrderBook,trade::Trade,traits::TradeListener};

pub struct MatchingEngine<T:TradeListener>
{
    order_book:SimpleOrderBook,
    trade_listener:T,
}

impl<T:TradeListener> MatchingEngine<T>
{
    pub fn new(trade_listener:T)->Self
    {
        Self 
        {
            order_book:SimpleOrderBook::new(),
            trade_listener,
        }
    }

    pub fn place_order(&mut self,order:Order)
    {
        let trades = self.order_book.add_order(order);
        for trade in &trades {
            self.trade_listener.on_trade(&trade);
        }
    }
}