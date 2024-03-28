use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult)]
pub struct SelectInvoicesItemsForUpdate {
    pub id: String,
    pub inventory_id: String,
    pub name: String,
    pub price: f64,
    pub quantity: f64,
    pub product_id: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, FromQueryResult)]
pub struct SelectInvoicesItems {
    pub name: String,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewInvoiceItem {
    pub invoice_id: String,
    pub inventory_id: String,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: String,
    pub invoice_id: String,
    pub inventory_id: String,
    pub price: f64,
}
