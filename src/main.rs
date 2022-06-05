extern crate env_logger;

mod infrastructure;
mod domain;

use std::sync::Mutex;
use crate::infrastructure::http::api::order::handler::{add_order, get_order, get_orders};
use crate::domain::order::repository::OrderRepository;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_web::web::Data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Leaving a comment here because of 3 hours of my life wasted (fucking multithreading)
    // We create our app data here
    let order_repository = OrderRepository::init();
    let order_data = Data::new(Mutex::new(order_repository));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // And we pass them to .app_data BY REFERENCE
            // and we NEED TO CLONE them to share them across threads
            .app_data(Data::clone(&order_data))
            .service(get_order)
            .service(add_order)
            .service(get_orders)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
