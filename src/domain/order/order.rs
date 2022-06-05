use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
enum OrderStatus {
    Open,
    Placed,
    Closed,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: i32,
    pub name: String,
    status: OrderStatus,
}

impl Order {
    pub fn create(id: i32, name: String) -> Order { Order { id, name, status: OrderStatus::Open } }

    pub fn set_open(&mut self) -> &mut Self { self.set_status(OrderStatus::Open) }

    pub fn set_placed(&mut self) -> &mut Self { self.set_status(OrderStatus::Placed) }

    pub fn set_closed(&mut self) -> &mut Self { self.set_status(OrderStatus::Closed) }

    pub fn is_open(&self) -> bool { self.has_status(OrderStatus::Open) }

    pub fn is_placed(&self) -> bool { self.has_status(OrderStatus::Placed) }

    pub fn is_closed(&self) -> bool { self.has_status(OrderStatus::Closed) }

    fn has_status(&self, status: OrderStatus) -> bool { self.status == status }

    fn set_status(&mut self, status: OrderStatus) -> &mut Self {
        self.status = status;

        self
    }
}
