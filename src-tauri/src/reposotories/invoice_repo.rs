use serde_json::{json, Value};

use crate::diesel::prelude::*;
use crate::models::{Client, Invoice, InvoiceItem, NewInvoice, Product, UpdateInvoice};
use crate::schema::{clients, invoice_items, invoices, products};

pub fn get_invoices(page: i32, connection: &mut SqliteConnection) -> Vec<Value> {
    let offset = (page - 1) * 17;

    let result = invoices::table
        .inner_join(clients::table.on(invoices::client_id.eq(clients::id)))
        .select((invoices::all_columns, clients::all_columns))
        .order(invoices::id.desc())
        .limit(17)
        .offset(offset as i64)
        .load::<(Invoice, Client)>(connection)
        .expect("Error fetching invoices with clients");

    result
        .into_iter()
        .map(|(invoice, client)| {
            let invoice_items: Vec<(InvoiceItem, Product)> = invoice_items::table
                .inner_join(products::table.on(invoice_items::product_id.eq(products::id)))
                .select((invoice_items::all_columns, products::all_columns))
                .filter(invoice_items::invoice_id.eq(invoice.id))
                .load::<(InvoiceItem, Product)>(connection)
                .expect("Error fetching invoice items with products");

            let invoice_items_json = json!({
                "invoiceItems": invoice_items.into_iter().map(|(item, product)| {
                    json!({
                        "id": item.id,
                        "quantity": item.quantity,
                        "product_id": item.product_id,
                        "inventory_id": item.inventory_id,
                        "product": {
                            "id": product.id,
                            "name": product.name,
                            "price": product.price
                        }
                    })
                }).collect::<Vec<_>>()
            });

            json!({
                "id": invoice.id,
                "status": invoice.status,
                "created_at": invoice.created_at,
                "client_id": invoice.client_id,
                "client": {
                    "id": client.id,
                    "fullname": client.fullname
                },
                "invoiceItems": invoice_items_json["invoiceItems"]
            })
        })
        .collect::<Vec<_>>()
}

pub fn get_invoice(i_id: i32, connection: &mut SqliteConnection) -> Value {
    let result = invoices::table
        .inner_join(clients::table.on(invoices::client_id.eq(clients::id)))
        .filter(invoices::id.eq(i_id))
        .select((invoices::all_columns, clients::all_columns))
        .load::<(Invoice, Client)>(connection)
        .expect("Error fetching invoices with clients");

    result
        .into_iter()
        .map(|(invoice, client)| {
            let invoice_items: Vec<(InvoiceItem, Product)> = invoice_items::table
                .inner_join(products::table.on(invoice_items::product_id.eq(products::id)))
                .select((invoice_items::all_columns, products::all_columns))
                .filter(invoice_items::invoice_id.eq(invoice.id))
                .load::<(InvoiceItem, Product)>(connection)
                .expect("Error fetching invoice items with products");

            println!("{:?}", invoice_items);
            let invoice_items_json = json!({
                "invoiceItems": invoice_items.into_iter().map(|(item, product)| {
                    json!({
                        "id": item.id,
                        "quantity": item.quantity,
                        "product_id": item.product_id,
                        "inventory_id": item.inventory_id,
                        "product": {
                            "id": product.id,
                            "name": product.name,
                            "price": product.price
                        }
                    })
                }).collect::<Vec<_>>()
            });

            json!({
                "id": invoice.id,
                "status": invoice.status,
                "created_at": invoice.created_at,
                "client_id": invoice.client_id,
                "client": {
                    "id": client.id,
                    "fullname": client.fullname
                },
                "invoiceItems": invoice_items_json["invoiceItems"]
            })
        })
        .collect::<Value>()
}

pub fn insert_invoice(new_i: NewInvoice, connection: &mut SqliteConnection) -> i32 {
    diesel::insert_into(invoices::dsl::invoices)
        .values(new_i)
        .execute(connection)
        .expect("Error adding invoice");

    let result = invoices::dsl::invoices
        .order_by(invoices::id.desc())
        .select(invoices::id)
        .first::<i32>(connection)
        .expect("error get all invoices");

    result
}

pub fn delete_invoice(i_id: i32, connection: &mut SqliteConnection) -> usize {
    let result = diesel::delete(invoices::dsl::invoices.find(&i_id))
        .execute(connection)
        .expect("Error deleting invoice");

    result
}

pub fn update_invoice(
    i_update: UpdateInvoice,
    i_id: i32,
    connection: &mut SqliteConnection,
) -> usize {
    let result = diesel::update(invoices::dsl::invoices.find(&i_id))
        .set((
            invoices::status.eq(i_update.status),
            // Add other fields here if needed
        ))
        .execute(connection)
        .expect("Error updating invoice");

    result
}
