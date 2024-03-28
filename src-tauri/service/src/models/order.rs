use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult)]
pub struct SelectOrders {
    pub id: String,
    pub created_at: String,
    pub supplier_id: String,
    pub full_name: String,
    pub status: String,
    pub products: i64,
    pub total: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult)]
pub struct SelectOrderDetails {
    pub id: String,
    pub created_at: String,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOrder {
    pub supplier_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub supplier_id: String,
    pub status: String,
}
