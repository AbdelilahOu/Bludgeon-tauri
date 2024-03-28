use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult)]
pub struct SelectInvoices {
    pub id: String,
    pub created_at: String,
    pub paid_amount: f64,
    pub client_id: String,
    pub full_name: String,
    pub status: String,
    pub products: i64,
    pub total: f64,
}


#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult)]
pub struct SelectInvoiceDetails {
    pub id: String,
    pub created_at: String,
    pub paid_amount: f64,
    pub full_name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewInvoice {
    pub client_id: String,
    pub status: String,
    pub paid_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub client_id: String,
    pub status: String,
    pub paid_amount: f64,
}
