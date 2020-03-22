use crate::orders::model::{Order};
use mysql_async::prelude::*;
use super::OrderConn;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderCreate {

}

impl OrderCreate {

    pub async fn from(mut order: Order) -> Result<Order, String> {
        if order.customer_id.is_none() && order.upload_id.is_none() && order.sku_id.is_none() {
            return Err("Missing required field:".to_string())
        }

        let database_url = OrderConn::get_url();

        let pool = mysql_async::Pool::new(database_url);
        let conn = pool.get_conn().await.unwrap();

        let params = params!{
            "order_id" => order.order_id,
            "customer_id" => order.customer_id,
            "payment_id "=> order.payment_id,
            "shipping_id" => order.shipping_id,
            "upload_id" => order.upload_id,
            "sku_id" => order.sku_id.clone(),
            "quantity" => order.quantity,
            "discount" => order.discount.clone(),
            "ready_to_ship" => order.ready_to_ship,
            "ready_on" => order.ready_on.clone(),
            "notes" => order.notes.clone(),
        };

        let insert_statement = OrderCreate::insert_statement_from(&order);

        let conn = conn.batch_exec(insert_statement, params).await.unwrap();
        order.order_id = Some(conn.last_insert_id().unwrap() as i32);

        // The destructor of a connection will return it to the pool,
        // but pool should be disconnected explicitly because it's
        // an asynchronous procedure.
        pool.disconnect().await.unwrap();

        Ok(order)
    }

    // Create a new record.  Must provide customer and upload ID!
    pub async fn new_item(customer_id: i32, upload_id: i32, sku_id: String) -> Result<Order, String> {
        let database_url = OrderConn::get_url();

        let pool = mysql_async::Pool::new(database_url);
        let conn = pool.get_conn().await.unwrap();

        let mut order = Order::new();
        order.customer_id = Some(customer_id);
        order.upload_id = Some(upload_id);
        order.sku_id = Some(sku_id);

        let orders = vec![ order.clone() ];
        let params = orders.into_iter().map(|order| {
            params!{
                "customer_id" => order.customer_id,
                "upload_id" => order.upload_id,
                "sku_id" => order.sku_id.clone(),
            }
        });

        match conn.batch_exec(r"INSERT INTO orderDB.item (customer_id, upload_id, sku_id)
                        VALUES (:customer_id, :upload_id, :sku_id)", params).await {
                            Ok(res) => {
                                println!("Inserted sample row {:?}.", res.last_insert_id().unwrap());
                            },
                            Err(err) => {
                                eprintln!("Unable to insert row: {:#?}", err);
                            },
                        }

        // The destructor of a connection will return it to the pool,
        // but pool should be disconnected explicitly because it's
        // an asynchronous procedure.
        pool.disconnect().await.unwrap();

        Ok(order)
    }

    pub fn insert_statement_from(order: &Order) -> String {
        let mut insert_statement: String = "INSERT INTO orderDB.item (".to_string();

        let mut insert_fields: String = "customer_id".to_string();
        let mut insert_values: String = ":customer_id".to_string();

        if order.payment_id.is_some() {
            insert_fields.push_str(", payment_id");
            insert_values.push_str(", :payment_id");
        }

        if order.shipping_id.is_some() {
            insert_fields.push_str(", shipping_id");
            insert_values.push_str(", :shipping_id");
        }

        insert_fields.push_str(", upload_id");
        insert_values.push_str(", :upload_id");

        insert_fields.push_str(", sku_id");
        insert_values.push_str(", :sku_id");

        if order.quantity.is_some() {
            insert_fields.push_str(", quantity");
            insert_values.push_str(", :quantity");
        }

        if order.discount.is_some() {
            insert_fields.push_str(", discount");
            insert_values.push_str(", :discount");
        }

        if order.ready_to_ship {
            insert_fields.push_str(", ready_to_ship");
            insert_values.push_str(", :ready_to_ship");
        }

        if order.notes.is_some() {
            insert_fields.push_str(", notes");
            insert_values.push_str(", :notes");
        }

        insert_statement.push_str(&insert_fields);
        insert_statement.push_str(") VALUES (");
        insert_statement.push_str(&insert_values);
        insert_statement.push_str(");");
        insert_statement
    }

    // Initialize a table if it doesn't exist with first two insert statment(one complete order and one default order).
    pub async fn create_table() -> Result<(), ()> {
        let mut builder = mysql_async::OptsBuilder::new();
        builder.ip_or_hostname("127.0.0.1".to_string());
        builder.user(Some("root".to_string()));

        let pool = mysql_async::Pool::new(builder);
        let conn = pool.get_conn().await.unwrap();

        let orders = vec![
            Order { order_id: None, customer_id: Some(1), payment_id: Some(1),
                    shipping_id: Some(1), upload_id: Some(1), sku_id: Some("gmask-wht-01-a".to_string()),
                    quantity: Some(1), discount: Some("".to_string()), ready_to_ship: true,
                    ready_on: Some("2020-03-21 23:54:08".to_string()), notes: Some("Seeded entry. No real sale.".to_string()),
                    created_on: Some("2020-03-21 23:54:08".to_string())
            },
        ];

        let orders_clone = orders.clone();
        let conn = match conn.drop_query("CREATE DATABASE orderDB").await {
            Ok(dbconn) => {
                println!("Created DB.");
                dbconn
            },
            Err(err) => {
                eprintln!("Cannot create DB: {:#?}", err);
                pool.get_conn().await.unwrap()
            },
        };

        let conn = match conn.drop_query(
            r"CREATE TABLE orderDB.item (
                order_id INT AUTO_INCREMENT PRIMARY KEY,
                customer_id INT NOT NULL,
                payment_id INT,
                shipping_id INT,
                upload_id INT NOT NULL,
                sku_id VARCHAR(255) NOT NULL,
                quantity INT,
                discount VARCHAR(255),
                ready_to_ship BOOLEAN NOT NULL DEFAULT FALSE,
                ready_on DATE,
                notes  VARCHAR(255),
                created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        ).await {
            Ok(dbconn) => {
                println!("Created DB table.");
                dbconn
            },
            Err(err) => {
                eprintln!("Cannot create DB table: {:#?}", err);
                pool.get_conn().await.unwrap()
            },
        };

        let params = orders_clone.into_iter().map(|order| {
            params! {
                "customer_id" => order.customer_id,
                "payment_id" => order.payment_id,
                "shipping_id" => order.customer_id,
                "upload_id" => order.upload_id,
                "sku_id" => order.sku_id,
                "quantity" => order.quantity,
                "discount" => order.discount,
                "ready_to_ship" => order.ready_to_ship,
                "ready_on" => order.ready_on,
                "notes" => order.notes.clone(),
            }
        });

        let insert_statement = OrderCreate::insert_statement_from(&orders[0]);

        match conn.batch_exec(insert_statement, params).await {
            Ok(res) => {
                println!("Inserted sample row {:?}.", res.last_insert_id().unwrap());
            },
            Err(err) => {
                eprintln!("Unable to insert row: {:#?}", err);
            },
        }

        Ok(())
    }

}
