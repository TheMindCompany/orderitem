use crate::orders::model::{Order};
use mysql_async::prelude::*;
use super::OrderConn;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderDelete {

}

impl OrderDelete {

    pub async fn remove_id(order_id: i32) -> Result<bool, String> {
        let database_url = OrderConn::get_url();

        let pool = mysql_async::Pool::new(database_url);
        let conn = pool.get_conn().await.unwrap();

        conn.drop_query(format!("UPDATE orderDB.item SET status='CANCELED' WHERE order_id={};", order_id))
            .await.unwrap();

        pool.disconnect().await.unwrap();

        Ok(true)
    }

}
