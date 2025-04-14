use std::collections::{BTreeMap,VecDeque};
use crate::{order::{Order,OrderSide},trade::Trade};

pub struct SimpleOrderBook
{
    pub bids:BTreeMap<u64,VecDeque<Order>>,
    pub asks:BTreeMap<u64,VecDeque<Order>>,
}

impl SimpleOrderBook{
    pub fn new()->Self 
    {
        Self 
        {
            bids:BTreeMap::new(),
            asks:BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self,order:Order)-> Vec<Trade>
    {
        let mut trades = Vec::new();
        let target_book= match order.side 
        {
            OrderSide::Buy => &mut self.asks,
            OrderSide::Sell => &mut self.bids,

        };

        let mut remaining_qty=order.quantity;

        let mut matched_prices= Vec::new();
        for (&price, orders) in target_book.iter_mut() {
            if (order.side == OrderSide::Buy && order.price >= price)
                || (order.side == OrderSide::Sell && order.price <= price)
            {
                while let Some(mut target_order) = orders.pop_front() {
                    let exec_qty = remaining_qty.min(target_order.quantity);
                    remaining_qty -= exec_qty;
                    target_order.quantity -= exec_qty;

                    trades.push(Trade {
                        buy_order_id: if order.side == OrderSide::Buy { order.id } else { target_order.id },
                        sell_order_id: if order.side == OrderSide::Sell { order.id } else { target_order.id },
                        price,
                        quantity: exec_qty,
                        timestamp: order.timestamp,
                    });

                    if target_order.quantity > 0 {
                        orders.push_front(target_order);
                        break;
                    }

                    if remaining_qty == 0 {
                        break;
                    }
                }
                matched_prices.push(price);
                if remaining_qty == 0 {
                    break;
                }
            }
        }

        for price in matched_prices {
            if let Some(q) = target_book.get(&price) {
                if q.is_empty() {
                    target_book.remove(&price);
                }
            }
        }

        if remaining_qty > 0 {
            let book = match order.side {
                OrderSide::Buy => &mut self.bids,
                OrderSide::Sell => &mut self.asks,
            };
            let entry = book.entry(order.price).or_insert_with(VecDeque::new);
            entry.push_back(Order { quantity: remaining_qty, ..order });
        }

        trades
    }

    pub fn cancel_order(&mut self,_order_id:u64)-> bool 
    {
        false
    }

    pub fn best_bid(&self)-> Option<&Order>
    {
        self.bids.iter().rev().next().and_then(|(_, v)| v.front())
    }

    pub fn full_depth(&self)-> (Vec<&Order>,Vec<&Order>)
    {
        let bids = self.bids.values().flat_map(|v| v.iter()).collect();
        let asks = self.asks.values().flat_map(|v| v.iter()).collect();
        (bids, asks)
    }

}