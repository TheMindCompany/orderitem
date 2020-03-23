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

        let pool = mysql_async::Pool::new(database_url);
        let conn = pool.get_conn().await.unwrap();

        // Load order from database. Because we need to reuse conn we want async func to return pool before next command.
        let result = conn.prep_exec(format!("SELECT order_id, status, customer_id, payment_id, shipping_id, upload_id, sku_id, quantity, discount, ready_to_ship, shipped_on, notes, created_on FROM orderDB.item WHERE order_id={};", order_id), ()).await.unwrap();
        let (_, order_vec) = result.map_and_drop(|row| {
            let mut return_order = Order::new();
            return_order.order_id = mysql_async::from_value(row.as_ref(0).unwrap().to_owned());
            return_order.status = mysql_async::from_value(row.as_ref(1).unwrap().to_owned());
            return_order.customer_id = mysql_async::from_value(row.as_ref(2).unwrap().to_owned());
            return_order.payment_id = mysql_async::from_value(row.as_ref(3).unwrap().to_owned());
            return_order.shipping_id = mysql_async::from_value(row.as_ref(4).unwrap().to_owned());
            return_order.upload_id = mysql_async::from_value(row.as_ref(5).unwrap().to_owned());
            return_order.sku_id = mysql_async::from_value(row.as_ref(6).unwrap().to_owned());
            return_order.quantity = mysql_async::from_value(row.as_ref(7).unwrap().to_owned());
            return_order.discount = mysql_async::from_value(row.as_ref(8).unwrap().to_owned());
            return_order.ready_to_ship = mysql_async::from_value(row.as_ref(9).unwrap().to_owned());
            return_order.shipped_on = mysql_async::from_value(row.as_ref(10).unwrap().to_owned());
            return_order.notes = mysql_async::from_value(row.as_ref(11).unwrap().to_owned());
            //return_order.created_on = mysql_async::from_value(row.as_ref(11).unwrap().to_owned());
            return_order
        }).await.unwrap();

        pool.disconnect().await.unwrap();

        if !order_vec.is_empty() { Ok(order_vec[0].clone()) } else { Err(format!("ORDER ID #{} => NOT FOUND", order_id)) }
    }

}
