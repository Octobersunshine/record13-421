use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub order_no: String,
    pub amount: f64,
    pub status: String,
    pub created_at: String,
    pub customer_name: String,
    pub customer_phone: String,
    pub customer_id_card: String,
    pub customer_email: String,
    pub bank_card_no: String,
    pub shipping_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: String,
    pub order_no: String,
    pub amount: f64,
    pub status: String,
    pub created_at: String,
    pub customer_name: String,
    pub customer_phone: String,
    pub customer_id_card: String,
    pub customer_email: String,
    pub bank_card_no: String,
    pub shipping_address: String,
}
