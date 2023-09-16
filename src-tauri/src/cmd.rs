use dotenv::dotenv;
use serde_json::Value;
use std::env;
use std::path;

use crate::csvparsing::export;
use crate::csvparsing::import;
use crate::csvparsing::import::TableRecord;
use crate::db;
use crate::models::*;
use crate::reposotories::*;
use crate::types::*;
use crate::AppState;

#[tauri::command]
pub async fn export_db_csv() -> String {
    dotenv().ok();
    let _env = env::var("DEV_ENV");
    let database_url: String = if _env.is_ok() == false {
        path::Path::new(&tauri::api::path::data_dir().unwrap())
            .join(".stocker")
            .join("stocker.db")
            .to_str()
            .expect("Failed to convert path to string")
            .to_string()
    } else {
        env::var("DATABASE_URL").expect("DATABASE_URL not set")
    };

    // table names
    let mut table_names: Vec<String> = vec![
        String::from("products"),
        String::from("clients"),
        String::from("sellers"),
        String::from("invoices"),
        String::from("orders"),
        String::from("inventory_mouvements"),
        String::from("order_items"),
        String::from("invoice_items"),
    ];
    // output path
    let documents_path = tauri::api::path::document_dir().unwrap();
    let output_path = path::Path::new(&documents_path);

    //
    for table in table_names.iter_mut() {
        // checking if we already have the csvs
        let out_put_file = output_path.join(format!("{}.csv", table));
        if out_put_file.exists() == false {
            // get data as csv
            export::table_to_csv(&database_url, &out_put_file.to_str().unwrap(), &table).await;
        }
    }

    String::from("OK")
}

#[tauri::command]
pub fn get_csv_records(csv_path: String, table: Option<String>) -> Result<TableRecord, String> {
    let result = import::get_csv_records(csv_path, table);
    result
}

#[tauri::command]
pub fn upload_csv_to_db(csv_path: String, table: String, state: tauri::State<AppState>) {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let records = get_csv_records(csv_path, Option::from(table));
    let result = db::insert_into_tables(records, conn);
    result
}

#[tauri::command]
pub async fn seed_db() {
    db::seed_db().await
}

#[tauri::command]
pub fn get_clients(page: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = client_repo::get_clients(page, conn);
    result
}

#[tauri::command]
pub fn get_all_clients(state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = client_repo::get_all_clients(conn);
    result
}

#[tauri::command]
pub fn get_client(id: i32, state: tauri::State<AppState>) -> Client {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = client_repo::get_client(id, conn);
    result
}

#[tauri::command]
pub fn delete_client(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = client_repo::delete_client(id, conn);
    result
}

#[tauri::command]
pub fn insert_client(client: NewClient, state: tauri::State<AppState>) -> Client {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = client_repo::insert_client(client, conn);
    result
}

#[tauri::command]
pub fn update_client(client: Client, id: i32, state: tauri::State<AppState>) -> Client {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = client_repo::update_client(client, id, conn);
    result
}

#[tauri::command]
pub fn get_sellers(page: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = seller_repo::get_sellers(page, conn);
    result
}

#[tauri::command]
pub fn get_all_sellers(state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = seller_repo::get_all_sellers(conn);
    result
}

#[tauri::command]
pub fn get_seller(id: i32, state: tauri::State<AppState>) -> Seller {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = seller_repo::get_seller(id, conn);
    result
}

#[tauri::command]
pub fn delete_seller(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = seller_repo::delete_seller(id, conn);
    result
}

#[tauri::command]
pub fn insert_seller(seller: NewSeller, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = seller_repo::insert_seller(seller, conn);
    result
}

#[tauri::command]
pub fn update_seller(seller: Seller, id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = seller_repo::update_seller(seller, id, conn);
    result
}

#[tauri::command]
pub fn get_products(page: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = product_repo::get_products(page, conn);
    result
}

#[tauri::command]
pub fn get_all_products(state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = product_repo::get_all_products(conn);
    result
}

#[tauri::command]
pub fn get_product(id: i32, state: tauri::State<AppState>) -> Product {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = product_repo::get_product(id, conn);
    result
}

#[tauri::command]
pub fn delete_product(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = product_repo::delete_product(id, conn);
    result
}

#[tauri::command]
pub fn insert_product(product: TNewProduct, state: tauri::State<AppState>) {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let id = product_repo::insert_product(
        NewProduct {
            description: product.description,
            name: product.name,
            price: product.price,
            tva: product.tva,
            image: product.image,
        },
        conn,
    );

    inventory_mvm_repo::insert_inventory_mvm(
        NewInventoryMvm {
            model: String::from("IN"),
            quantity: product.quantity,
            product_id: id,
        },
        conn,
    );
}

#[tauri::command]
pub fn update_product(product: Product, id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = product_repo::update_product(product, id, conn);
    result
}

#[tauri::command]
pub fn get_user(id: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = user_repo::get_user(id, conn);
    result
}

#[tauri::command]
pub fn delete_user(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = user_repo::delete_user(id, conn);
    result
}

#[tauri::command]
pub fn insert_user(user: NewUser, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = user_repo::insert_user(user, conn);
    result
}

#[tauri::command]
pub fn update_user(user: User, id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = user_repo::update_user(user, id, conn);
    result
}

#[tauri::command]
pub fn get_invoice(id: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = invoice_repo::get_invoice(id, conn);
    result
}

#[tauri::command]
pub fn get_invoices(page: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = invoice_repo::get_invoices(page, conn);
    result
}

#[tauri::command]
pub fn delete_invoice(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = invoice_repo::delete_invoice(id, conn);
    result
}

#[tauri::command]
pub fn insert_invoice(invoice: TNewInvoice, state: tauri::State<AppState>) {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let inserted_id = invoice_repo::insert_invoice(
        NewInvoice {
            status: invoice.status,
            client_id: invoice.client_id,
        },
        conn,
    );
    for item in invoice.invoice_items.into_iter() {
        let inserted_im_id = inventory_mvm_repo::insert_inventory_mvm(
            NewInventoryMvm {
                model: String::from("OUT"),
                quantity: item.quantity,
                product_id: item.product_id,
            },
            conn,
        );

        invoice_item_repo::insert_invoice_item(
            NewInvoiceItem {
                product_id: item.product_id,
                invoice_id: inserted_id.clone(),
                quantity: item.quantity,
                inventory_id: inserted_im_id,
            },
            conn,
        );
    }
}

#[tauri::command]
pub fn update_invoice(invoice: TUpdateInvoice, id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = invoice_repo::update_invoice(
        UpdateInvoice {
            status: invoice.status,
        },
        id,
        conn,
    );
    for item in invoice.invoice_items.into_iter() {
        println!("{:?}", item);
        match item.id {
            Some(ii_id) => match item.inventory_id {
                Some(im_id) => {
                    invoice_item_repo::update_invoice_item(
                        UpdateInvoiceItem {
                            quantity: item.quantity,
                        },
                        ii_id,
                        conn,
                    );

                    inventory_mvm_repo::update_inventory_mvm(
                        UpdateInventoryMvm {
                            quantity: item.quantity,
                        },
                        im_id,
                        conn,
                    );
                }
                None => println!("No inventory id"),
            },

            None => {
                let inserted_im_id = inventory_mvm_repo::insert_inventory_mvm(
                    NewInventoryMvm {
                        model: String::from("OUT"),
                        quantity: item.quantity,
                        product_id: item.product_id,
                    },
                    conn,
                );

                invoice_item_repo::insert_invoice_item(
                    NewInvoiceItem {
                        product_id: item.product_id,
                        invoice_id: id,
                        quantity: item.quantity,
                        inventory_id: inserted_im_id,
                    },
                    conn,
                );
            }
        }
    }
    result
}

#[tauri::command]
pub fn get_order(id: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = order_repo::get_order(id, conn);
    result
}

#[tauri::command]
pub fn get_orders(page: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = order_repo::get_orders(page, conn);
    result
}

#[tauri::command]
pub fn delete_order(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = order_repo::delete_order(id, conn);
    result
}

#[tauri::command]
pub fn insert_order(order: TNewOrder, state: tauri::State<AppState>) {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let inserted_id = order_repo::insert_order(
        NewOrder {
            status: order.status,
            seller_id: order.seller_id,
        },
        conn,
    );
    for item in order.order_items.into_iter() {
        let inserted_im_id = inventory_mvm_repo::insert_inventory_mvm(
            NewInventoryMvm {
                model: String::from("IN"),
                quantity: item.quantity,
                product_id: item.product_id,
            },
            conn,
        );

        order_item_repo::insert_order_item(
            NewOrderItem {
                product_id: item.product_id,
                order_id: inserted_id.clone(),
                quantity: item.quantity,
                price: item.price,
                inventory_id: inserted_im_id,
            },
            conn,
        );
    }
}

#[tauri::command]
pub fn update_order(order: TUpdateOrder, id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = order_repo::update_order(
        UpdateOrder {
            status: order.status,
        },
        id,
        conn,
    );

    for item in order.order_items.into_iter() {
        match item.id {
            Some(oi_id) => match item.inventory_id {
                Some(im_id) => {
                    order_item_repo::update_order_item(
                        UpdateOrderItem {
                            quantity: item.quantity,
                            price: item.price,
                        },
                        oi_id,
                        conn,
                    );

                    inventory_mvm_repo::update_inventory_mvm(
                        UpdateInventoryMvm {
                            quantity: item.quantity,
                        },
                        im_id,
                        conn,
                    );
                }
                None => println!("No inventory id"),
            },

            None => {
                let inserted_im_id = inventory_mvm_repo::insert_inventory_mvm(
                    NewInventoryMvm {
                        model: String::from("OUT"),
                        quantity: item.quantity,
                        product_id: item.product_id,
                    },
                    conn,
                );

                order_item_repo::insert_order_item(
                    NewOrderItem {
                        product_id: item.product_id,
                        order_id: id,
                        quantity: item.quantity,
                        price: item.price,
                        inventory_id: inserted_im_id,
                    },
                    conn,
                );
            }
        }
    }
    result
}

#[tauri::command]
pub fn get_order_items(id: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = order_item_repo::get_order_items(id, conn);
    result
}

#[tauri::command]
pub fn delete_order_items(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = order_item_repo::delete_order_item(id, conn);
    result
}

#[tauri::command]
pub fn get_invoice_items(id: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = invoice_item_repo::get_invoice_items(id, conn);
    result
}

#[tauri::command]
pub fn delete_invoice_items(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = invoice_item_repo::delete_invoice_item(id, conn);
    result
}

#[tauri::command]
pub fn get_inventory_mvms(page: i32, state: tauri::State<AppState>) -> Value {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = inventory_mvm_repo::get_inventory(page, conn);
    result
}

#[tauri::command]
pub fn insert_inventory_mvm(inventory: TNewInventory, state: tauri::State<AppState>) -> i32 {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = inventory_mvm_repo::insert_inventory_mvm(
        NewInventoryMvm {
            model: inventory.model,
            quantity: inventory.quantity,
            product_id: inventory.product_id,
        },
        conn,
    );

    result
}

#[tauri::command]
pub fn delete_inventory_mvm(id: i32, state: tauri::State<AppState>) -> usize {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = inventory_mvm_repo::delete_inventory_mvm(id, conn);
    result
}

#[tauri::command]
pub fn get_b3_sellers(state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = stats_repo::get_best_three_seller(conn);
    result
}

#[tauri::command]
pub fn get_b3_clients(state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = stats_repo::get_best_three_client(conn);
    result
}

#[tauri::command]
pub fn get_c_week_expenses(id: i32, state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = stats_repo::get_client_expenses(id, conn);
    result
}

#[tauri::command]
pub fn get_s_week_expenses(id: i32, state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = stats_repo::get_seller_expenses(id, conn);
    result
}

#[tauri::command]
pub fn get_c_product_month(id: i32, state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = stats_repo::get_client_details(id, conn);
    result
}

#[tauri::command]
pub fn get_s_product_month(id: i32, state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = stats_repo::get_seller_details(id, conn);
    result
}

#[tauri::command]
pub fn get_inventory_stats(state: tauri::State<AppState>) -> Vec<Value> {
    let mut conn = state.db_conn.lock().unwrap();
    let conn = &mut *conn;
    let result = stats_repo::get_inventory_stats(conn);
    result
}
