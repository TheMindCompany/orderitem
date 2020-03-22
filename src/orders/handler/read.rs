use mysql_async::Conn;
use crate::orders::model::{Order};
use mysql_async::prelude::*;
use super::OrderConn;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderRead {

}

impl OrderRead {

    // Get an Order object eith its id.
    pub async fn get_id(order_id: i32) -> Result<Order, String> {
        let database_url = OrderConn::get_url();
        let mut return_order = Order::new();

        let pool = mysql_async::Pool::new(database_url);
        let conn = pool.get_conn().await.unwrap();

        // Load order from database. Because we need to reuse conn we want async func to return pool before next command.
        let result = conn.query(format!("SELECT * FROM orderDB.item WHERE order_id={}", order_id)).await.unwrap();
        let (_, res): (Conn, Vec<(Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<String>, bool, Option<String>, Option<String>, Option<String>)>) = result.collect_and_drop().await.unwrap();

        return_order.order_id = res[0].0;
        return_order.customer_id = res[0].1;
        return_order.payment_id = res[0].2;
        return_order.shipping_id = res[0].3;
        return_order.upload_id = res[0].4;
        return_order.sku_id = res[0].5.clone();
        return_order.quantity = res[0].6;
        return_order.discount = res[0].7.clone();
        return_order.ready_to_ship = res[0].8;
        return_order.ready_on = res[0].9.clone();
        return_order.notes = res[0].10.clone();
        return_order.created_on = res[0].11.clone();
        
        // The destructor of a connection will return it to the pool,
        // but pool should be disconnected explicitly because it's
        // an asynchronous procedure.
        pool.disconnect().await.unwrap();

        Ok(return_order)
    }

}
