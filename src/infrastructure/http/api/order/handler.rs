use std::borrow::BorrowMut;
use std::sync::Mutex;
use actix_web::{get, post, web::Json, web::Data};
use actix_web::web::Path;
use serde::{Deserialize, Serialize};

use crate::domain::order::order::{Order};
use crate::infrastructure::http::api::order::error::OrderError;
use crate::domain::order::repository::{OrderRepository, OrderRepositoryTrait};

#[derive(Deserialize, Serialize)]
pub struct OrderIdentifier {
    id: i32
}

#[derive(Deserialize)]
pub struct AddOrderRequest {
    id: i32,
    name: String
}

#[get("/order/{id}")]
pub async fn get_order(
    repo: Data<Mutex<OrderRepository>>,
    id: Path<OrderIdentifier>,
) -> Result<Json<Order>, OrderError> {
    match repo.lock().unwrap().get_order(id.into_inner().id) {
        Ok(o) => Ok(Json(o)),
        Err(_e) => Err(OrderError::OrderNotFound)
    }
}

#[post("/order")]
pub async fn add_order(
    repo: Data<Mutex<OrderRepository>>,
    request: Json<AddOrderRequest>,
) -> Result<Json<()>, OrderError> {
    let order = Order::create(request.id.clone(), request.name.clone());

    match repo.lock().unwrap().borrow_mut().add_order(order) {
        Ok(_o) => Ok(Json(())),
        Err(_e) => Err(OrderError::OrderAlreadyExists),
    }
}

#[get("/order")]
pub async fn get_orders(
    repo: Data<Mutex<OrderRepository>>
) -> Result<Json<Vec<Order>>, OrderError> {
    Ok(Json(Vec::from_iter(repo.lock().unwrap().get_orders().values().cloned())))
}
