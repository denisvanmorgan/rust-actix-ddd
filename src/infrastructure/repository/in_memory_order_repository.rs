use std::collections::HashMap;
use Result;
use crate::domain::order::order::Order;
use crate::domain::order::repository::{OrderRepositoryTrait, OrderRepository};

impl OrderRepositoryTrait for OrderRepository {
    fn add_order(&mut self, order: Order) -> Result<(), String> {
        if self.orders.contains_key(&order.id) {
            return Err("Order already exists".to_string());
        }

        self.orders.insert(order.id, order);

        return Ok(());
    }

    fn get_order(&self, id: i32) -> Result<Order, String> {
        let order = self.orders.get(&id).cloned();

        match order {
            Some(o) => Ok(o),
            None => Err("Order not found".to_string())
        }
    }

    fn get_orders(&self) -> HashMap<i32, Order> {
        self.orders.clone()
    }
}
