use crate::orders::model::{Order};
use mysql_async::prelude::*;
use super::OrderConn;

use std::{
    collections::VecDeque,
    fmt,
    pin::Pin,
    str::FromStr,
    sync::{atomic, Arc, Mutex},
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderUpdate {

}

impl OrderUpdate {

    // This is a quick way to update.  Create a default Order object and update with only needed
    // values and this function will resolve into the proper update statement.
    pub async fn with_order(order: Order) -> Result<Order, String> {
        if order.order_id.is_none() {
            return Err("No Order ID".to_string())
        }
        //let database_url = OrderConn::get_url();
        let mut builder = mysql_async::OptsBuilder::new();
        builder.ip_or_hostname("127.0.0.1".to_string());

        let pool = mysql_async::Pool::new(builder);
        let conn = pool.get_conn().await.unwrap();
        let params = params!{
            "order_id" => OrderUpdate::unwrap_to_i32(order.order_id),
            "customer_id" => OrderUpdate::unwrap_to_i32(order.customer_id),
            "payment_id "=> OrderUpdate::unwrap_to_i32(order.payment_id),
            "shipping_id" => OrderUpdate::unwrap_to_i32(order.shipping_id),
            "upload_id" => OrderUpdate::unwrap_to_i32(order.upload_id),
            "sku_id" => OrderUpdate::unwrap_to_string(order.sku_id.clone()),
            "quantity" => OrderUpdate::unwrap_to_i32(order.quantity),
            "discount" => OrderUpdate::unwrap_to_string(order.discount.clone()),
            "ready_to_ship" => order.ready_to_ship,
            "ready_on" => OrderUpdate::unwrap_to_string(order.ready_on.clone()),
            "notes" => OrderUpdate::unwrap_to_string(order.notes.clone()),
        };
        let mut update_statement: String = "UPDATE order.item SET customer_id=:customer_id".to_string();

        if order.payment_id.is_some() {
            update_statement.push_str(", payment_id=:payment_id");
        }

        if order.shipping_id.is_some() {
            update_statement.push_str(", shipping_id=:shipping_id");
        }

        if order.upload_id.is_some() {
            update_statement.push_str(", upload_id=:upload_id");
        }

        if order.sku_id.is_some() {
            update_statement.push_str(", sku_id=:sku_id");
        }

        if order.quantity.is_some() {
            update_statement.push_str(", quantity=:quantity");
        }

        if order.discount.is_some() {
            update_statement.push_str(", discount=:discount");
        }

        if order.ready_on.is_some() {
            update_statement.push_str(", ready_on=:ready_on");
        }

        if order.ready_to_ship {
            update_statement.push_str(", ready_to_ship=:ready_to_ship");
        }

        if order.notes.is_some() {
            update_statement.push_str(", notes=:notes");
        }

        update_statement.push_str(" WHERE order_id=:order_id;");

        conn.batch_exec(update_statement, params).await.unwrap();

        // The destructor of a connection will return it to the pool,
        // but pool should be disconnected explicitly because it's
        // an asynchronous procedure.
        pool.disconnect().await.unwrap();

        Ok(order)
    }

    pub fn unwrap_to_string(val: Option<String>) -> String {
        let mut response = String::new();
        if let Some(content) = val {
            response = content;
        }

        response
    }

    pub fn unwrap_to_i32(val: Option<i32>) -> i32 {
        let mut response = 0;
        if let Some(content) = val {
            response = content;
        }

        response
    }

}
