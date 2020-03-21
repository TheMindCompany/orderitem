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

        let pool = mysql_async::Pool::new(database_url.await);
        let conn = pool.get_conn().await.unwrap();

        // Load order from database. Because we need to reuse conn we want async func to return pool before next command.
        let result = conn.prep_exec("SELECT * FROM order.item WHERE order_id={:order_id};", params!{"order_id" => order_id}).await.unwrap();
        let (_, loaded_orders) = result.map_and_drop(|row| {
            let order: (Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<String>, Option<i32>, Option<String>, bool, Option<String>, Option<String>, Option<String>) = mysql_async::from_row_opt(row).unwrap();
            Order {
                order_id: order.0,
                customer_id: order.1,
                payment_id: order.2,
                shipping_id: order.3,
                upload_id: order.4,
                sku_id: order.5,
                quantity: order.6,
                discount: order.7,
                ready_to_ship: order.8,
                ready_on: order.9,
                notes: order.10,
                created_on: order.11,
            }
        }).await.unwrap();
        if loaded_orders.len() == 1 {
            for current_order in loaded_orders {
                println!("Recorder found: {:#?}", current_order);
                return_order = current_order;
            }
        } else {
            eprintln!("There is a problem creating a new order from order#{:#?}.", order_id);
            if loaded_orders.is_empty() {
                eprintln!("Oiginal order NOT FOUND");
            } else {
                eprintln!("More than one order with id. (HOW THE FUCK???)");
            }
        }

        // The destructor of a connection will return it to the pool,
        // but pool should be disconnected explicitly because it's
        // an asynchronous procedure.
        pool.disconnect().await.unwrap();

        Ok(return_order)
    }

}
