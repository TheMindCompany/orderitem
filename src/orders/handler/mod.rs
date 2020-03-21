pub mod create;
pub mod read;
pub mod update;
pub mod delete;

pub use create::OrderCreate;
pub use read::OrderRead;
pub use update::OrderUpdate;
pub use delete::OrderDelete;
use mysql_async::{Opts, OptsBuilder};

use std::{env, u16};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderConn {}

impl OrderConn {
    pub async fn get_url() -> Opts {
        let conn_str = OrderConn::from_env().await;
        let opts = Opts::from(conn_str);

        if opts.get_db_name().expect("a database name is required").is_empty() {
            panic!("database name is empty");
        }

        opts
    }

    pub async fn from_env() -> OptsBuilder {
        let mut builder = mysql_async::OptsBuilder::new();
        let port: u16 = if env::var("ORDERITEM_SQL_PORT").is_ok() { env::var("ORDERITEM_SQL_PORT").unwrap().parse().unwrap() } else { 3306 };

        builder.ip_or_hostname(env::var("ORDERITEM_SQL_HOST").unwrap());
        builder.tcp_port(port);
        builder.db_name(Some("orders".to_string()));
        if let Ok(host_username) = env::var("ORDERITEM_SQL_USERNAME") {
            if host_username != "None" {
                builder.user(Some(host_username));

                if let Ok(password) = env::var("ORDERITEM_SQL_PASSWORD") {
                    if password != "None" {
                        builder.pass(Some(password));
                    }
                };
            }
        }

        builder
    }
}
