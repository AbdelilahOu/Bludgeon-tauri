#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;

use db::establish_connection;
use migration::{Migrator, MigratorTrait};
use service::sea_orm::DatabaseConnection;

pub struct AppState {
    db_conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    // establish conn
    let db_conn = establish_connection().await;
    // run migrations
    Migrator::up(&db_conn, None).await.unwrap();
    //
    tauri::Builder::default()
        .manage(AppState { db_conn })
        .invoke_handler(tauri::generate_handler![
            commands::products::list_products,
            commands::products::search_products,
            commands::products::create_product,
            commands::products::update_product,
            commands::products::delete_product,
        ])
        .plugin(tauri_plugin_oauth::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
