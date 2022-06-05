use std::collections::HashMap;
use crate::domain::order::order::Order;

pub struct OrderRepository {
    pub orders: HashMap<i32, Order>
}

impl OrderRepository {
    pub fn init() -> OrderRepository {
        OrderRepository { orders: HashMap::new() }
    }
}

pub trait OrderRepositoryTrait {
    fn add_order(&mut self, order: Order) -> Result<(), String>;
    fn get_order(&self, id: i32) -> Result<Order, String>;
    fn get_orders(&self) -> HashMap<i32, Order>;
}
