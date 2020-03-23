use crate::orders::model::{Order};
use mysql_async::prelude::*;
use super::OrderConn;

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

        let database_url = OrderConn::get_url();

        let pool = mysql_async::Pool::new(database_url);
        let conn = pool.get_conn().await.unwrap();
        let orders = vec![ order.clone() ];

        let params = orders.into_iter().map(|order| {
            params! {
                "order_id" => order.order_id,
                "status" => order.status,
                "customer_id" => order.customer_id,
                "payment_id" => order.payment_id,
                "shipping_id" => order.customer_id,
                "upload_id" => order.upload_id,
                "sku_id" => order.sku_id,
                "quantity" => order.quantity,
                "discount" => order.discount,
                "ready_to_ship" => order.ready_to_ship,
                "shipped_on" => order.shipped_on,
                "notes" => order.notes,
                "created_on" => order.created_on.clone(),
            }
        });

        let mut update_statement: String = "UPDATE orderDB.item SET".to_string();

        if order.status.is_some() {
            update_statement.push_str(" status=:status");
        }

        if order.customer_id.is_some() {
            update_statement.push_str(" customer_id=:customer_id");
        }

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

        if order.shipped_on.is_some() {
            update_statement.push_str(", shipped_on=:shipped_on");
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

}
