#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;

// modes
mod csvparsing;
mod db;
mod models;
mod queries;
mod schema;

use crate::csvparsing::{export::*, import::*};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_csv_records, export_db_csv])
        .plugin(tauri_plugin_oauth::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
