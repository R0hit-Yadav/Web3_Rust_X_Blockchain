#[derive(Debug,Clone)]
pub struct Trade{
    pub buy_order_id:u64,
    pub sell_order_id:u64,
    pub price:u64,
    pub quantity:u64,
    pub timestamp:u64,
}