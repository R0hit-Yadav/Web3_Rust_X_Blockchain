#[derive(Debug, Clone, PartialEq,Eq)]
pub enum OrderSide
{
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType
{
    Limit,
    Market,
    PostOnly,
    ImmediateOrCancel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order
{
    pub id: u64,
    pub user_id: u64,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u64,
}