use std::iter;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Order {
    pub order_id: Option<i32>,
     // Must provide.
    pub customer_id: Option<i32>,
    // None until it is payed.
    pub payment_id: Option<i32>,
    // None until it is shipped.
    pub shipping_id: Option<i32>,
    // Must provide upload to create an order. Should have some kind of ttl on order_id if upload_id is None.
    pub upload_id: Option<i32>,
    // The model sku id.
    pub sku_id: Option<String>,
    // How many of item are to be ordered.
    pub quantity: Option<i32>,
    // How many of item are to be ordered.
    pub discount: Option<String>,
    // Is order printed and ready to be shipped.
    pub ready_to_ship: bool,
    // Any service notes to assit fufilment.
    pub ready_on: Option<String>,
    // Any service notes to assit fufilment.
    pub notes: Option<String>,
    pub created_on: Option<String>,
}

impl Order {
    pub fn new() -> Order {
        Default::default()
    }

    pub fn values(&self) -> impl Iterator<Item = Option<i32>> {
        let order_id = iter::once(self.order_id);
        let customer_id = iter::once(self.customer_id);
        let payment_id = iter::once(self.payment_id);
        let shipping_id = iter::once(self.shipping_id);
        let upload_id = iter::once(self.upload_id);
        order_id.chain(customer_id).chain(payment_id).chain(shipping_id).chain(upload_id)
    }
}
