use actix_web::{HttpResponse, error::ResponseError, http::StatusCode, http::header::ContentType};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum OrderError {
    #[display(fmt = "Order not found")]
    OrderNotFound,

    #[display(fmt = "Order not found")]
    OrderAlreadyExists,
}

impl ResponseError for OrderError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match self {
            OrderError::OrderNotFound => StatusCode::NOT_FOUND,
            OrderError::OrderAlreadyExists => StatusCode::FORBIDDEN,
        }
    }
}
