use crate::orders::Order;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderItemResponse {
    pub data: OrderItemData,
    pub error: Option<String>,
}

impl OrderItemResponse {

    pub fn new() -> OrderItemResponse {
        Default::default()
    }

    pub fn set_attributes(&mut self, val: Order) {
        self.data.set_attributes(val);
    }

    pub fn set_error(&mut self, val: String) {
        self.error = Some(val);
    }

}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct OrderItemData {
    pub attributes: Order,
}

impl OrderItemData {

    pub fn set_attributes(&mut self, val: Order) {
        self.attributes = val;
    }

}
