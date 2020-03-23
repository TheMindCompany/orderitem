use crate::orders::Order;
use std::fmt::{Display, Formatter, Result as FmtResult};

use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
use serde::Serialize;
use serde_json::{json, to_string_pretty};

#[derive(Debug, Serialize)]
pub struct OrderItemError {
    // id: i32,
    // links: String,
    // about: String,
    // type: String,
    pub status: u16,
    // code: i32,
    // title: String,
    // detail: String,
    // source: String,
    // pointer: String,
    // parameter: String,
    // meta: String,
    pub msg: String,
}

impl Display for OrderItemError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for OrderItemError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> web::HttpResponse {
        let err_json = json!({ "error": self.msg });
        web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .json(err_json)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderItemResponse {
    pub data: OrderItemData,
}

impl OrderItemResponse {

    pub fn new() -> OrderItemResponse {
        Default::default()
    }

    pub fn set_attributes(&mut self, val: Order) {
        self.data.set_attributes(val);
    }

}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderItemData {
    pub attributes: Order,
}

impl OrderItemData {

    pub fn set_attributes(&mut self, val: Order) {
        self.attributes = val;
    }

}
