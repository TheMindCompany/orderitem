pub mod model;
pub mod handler;

use crate::command_control::cmd_model::cmdctl::{CmdCtl, MethodType};
pub use model::*;
pub use handler::*;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderItemRunner {}

impl OrderItemRunner {
    pub async fn run(request: &CmdCtl) -> Result<Order, String> {
        let dry_run = request.dry_run;
        let verbose = request.verbose;

        match request.method() {
            MethodType::CreateEntry => {
                if request.order_id.is_some() {
                    match OrderCreate::from(request.order_detail()).await {
                        Ok(order) => {
                            Ok(order)
                        },
                        Err(err) => {
                            Err(err)
                        },
                    }
                } else {
                    match OrderCreate::new_item(
                        request.customer_id.unwrap(),
                        request.upload_id.unwrap(),
                        request.sku_id.clone().unwrap()
                    ).await {
                        Ok(order) => {
                            Ok(order)
                        },
                        Err(err) => {
                            Err(err)
                        },
                    }
                }
            },
            MethodType::ReadEntry => {
                match OrderRead::get_id(request.order_id.unwrap()).await {
                    Ok(order) => {
                        Ok(order)
                    },
                    Err(err) => {
                        Err(err)
                    },
                }
            },
            MethodType::UpdateEntry => {
                match OrderUpdate::with_order(request.order_detail()).await {
                    Ok(order) => {
                        Ok(order)
                    },
                    Err(err) => {
                        Err(err)
                    },
                }
            },
            MethodType::DeleteEntry => {
                match OrderDelete::remove_id(request.order_id.unwrap()).await {
                    Ok(complete) => {
                        if complete { Ok(Order::new()) } else { Err("Could not delete order.".to_string()) }
                    },
                    Err(err) => {
                        Err(err)
                    },
                }
            },
        }
    }
}
