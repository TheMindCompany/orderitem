use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use super::Commands;
use crate::orders::Order;

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(
    global_settings = &[DisableVersion, DeriveDisplayOrder, VersionlessSubcommands],
    about = "Create, Read, Update, Delete order items."
)]
pub struct CmdCtl {
    /// The type of method being requested for signing url.
    #[structopt(default_value="READ")]
    pub method: String,

    /// Order ID
    #[structopt(short = "o", long = "order-id")]
    pub order_id: Option<i32>,

    /// Order status.
    #[structopt(long = "status")]
    pub status: Option<String>,

    /// Customer ID
    #[structopt(short = "c", long = "customer-id")]
    pub customer_id: Option<i32>,

    /// Payment ID
    #[structopt(long = "payment-id")]
    pub payment_id: Option<i32>,

    /// Shipping ID
    #[structopt(long = "shipping-id")]
    pub shipping_id: Option<i32>,

    /// Upload ID
    #[structopt(short = "u", long = "upload-id")]
    pub upload_id: Option<i32>,

    /// Sku Model ID
    #[structopt(short = "s", long = "sku")]
    pub sku_id: Option<String>,

    /// How many items.
    #[structopt(short = "q", long = "quantity")]
    pub quantity: Option<i32>,

    /// Apply discount
    #[structopt(long = "discount")]
    pub discount: Option<String>,

    /// Remove discount
    #[structopt(long = "remove-discount")]
    pub remove_discount: Option<String>,

    /// Mark ready to ship order.
    #[structopt(short = "r", long = "ready-to-ship")]
    pub ready_to_ship: bool,

    /// Add/Append a note for order.
    ///
    ///
    #[structopt(short = "n", long = "notes")]
    pub notes: Option<String>,

    /// Seed database on initialization if it doesn't exist.
    #[structopt(long = "seed")]
    pub seed: bool,

    /// Request time to live in milliseconds.
    #[structopt(short = "t", long = "timeout", default_value="60000")]
    pub ttl: u64,

    /// Daemon mode.
    #[structopt(short = "d", long = "daemon")]
    pub daemon: bool,

    /// Daemon mode port.
    #[structopt(long = "port", env = "OrderItem_PORT", default_value="8080")]
    pub port: i32,

    /// Daemon mode host.
    #[structopt(long = "host", env = "OrderItem_HOST", default_value="127.0.0.1")]
    pub host: String,

    /// Don't run commands only log.
    #[structopt(long = "dry-run")]
    pub dry_run: bool,

    /// Enable verbose logging.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

pub enum MethodType {
    CreateEntry,
    ReadEntry,
    UpdateEntry,
    DeleteEntry,
}
pub struct CreateEntry {}
pub struct ReadEntry {}
pub struct UpdateEntry {}
pub struct DeleteEntry {}

impl CmdCtl {

    pub fn run_command_process(self) -> CmdCtl {
        match &self.commands {
            Some(commands) => {
                commands.process();
                self
            },
            None => {
                self
            },
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self.commands.clone() {
            Some(commands) => commands.is_verbose(),
            None => self.verbose
        }
    }

    pub fn method(&self) -> MethodType {
        match self.method.to_uppercase().as_str() {
            "CREATE" | "ADD" | "POST" => MethodType::CreateEntry,
            "READ" | "GET" => MethodType::ReadEntry,
            "UPDATE" | "PUT" => MethodType::UpdateEntry,
            "DELETE" | "DEL" => MethodType::DeleteEntry,
            _ => MethodType::ReadEntry,
        }
    }

    pub fn order_detail(&self) -> Order {
        let mut order = Order::new();

        if self.order_id.is_some() {
            order.order_id = self.order_id;
        }

        if self.customer_id.is_some() {
            order.customer_id = self.customer_id;
        }

        if self.status.is_some() {
            order.status = self.status.clone();
        }

        if self.payment_id.is_some() {
            order.payment_id = self.payment_id;
        }

        if self.shipping_id.is_some() {
            order.shipping_id = self.shipping_id;
        }

        if self.upload_id.is_some() {
            order.upload_id = self.upload_id;
        }

        if self.sku_id.is_some() {
            order.sku_id = self.sku_id.clone();
        }

        if self.quantity.is_some() {
            order.quantity = self.quantity;
        }

        if self.discount.is_some() {
            order.discount = self.discount.clone();
        }

        if self.ready_to_ship {
            order.ready_to_ship = self.ready_to_ship;
        }

        if self.notes.is_some() {
            order.notes = self.notes.clone();
        }

        order
    }

}
