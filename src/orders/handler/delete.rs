use crate::orders::model::{Order};
use mysql_async::prelude::*;
use super::OrderConn;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderDelete {

}

impl OrderDelete {

    pub async fn remove_id(order_id: i32) -> Result<bool, String> {
        let database_url = OrderConn::get_url();

        let pool = mysql_async::Pool::new(database_url.await);
        let conn = pool.get_conn().await.unwrap();

        conn.batch_exec(r"DELETE order.item (order_id)
                        VALUES (:order_id)", params!{"order_id" => order_id}).await.unwrap();

        // The destructor of a connection will return it to the pool,
        // but pool should be disconnected explicitly because it's
        // an asynchronous procedure.
        pool.disconnect().await.unwrap();

        Ok(true)
    }

}
