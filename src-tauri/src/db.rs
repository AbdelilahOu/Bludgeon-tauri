use migration::sea_orm::Database;
use migration::sea_orm::DatabaseConnection;

#[cfg(not(debug_assertions))]
use std::fs;

#[cfg(debug_assertions)]
use dotenvy::dotenv;
#[cfg(debug_assertions)]
use std::env;

pub async fn establish_connection() -> DatabaseConnection {
    #[cfg(debug_assertions)]
    dotenv().ok();

    #[cfg(debug_assertions)]
    let db_url = env::var("DATABASE_URL").unwrap();

    #[cfg(not(debug_assertions))]
    let home_dir = match tauri::api::path::data_dir() {
        Some(val) => val,
        None => panic!("Could not get home directory"),
    };

    #[cfg(not(debug_assertions))]
    let data_dir = home_dir.join(".mahali/data");
    #[cfg(not(debug_assertions))]
    if let Err(_) = fs::metadata(&data_dir) {
        fs::create_dir_all(&data_dir).expect("Could not create data directory");
    }

    #[cfg(not(debug_assertions))]
    let db_url = "sqlite://".to_string() + data_dir.to_str().unwrap() + "/db.sqlite?mode=rwc";

    Database::connect(&db_url)
        .await
        .expect(&format!("Error connecting to {}", &db_url))
}
