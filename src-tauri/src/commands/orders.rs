use serde_json::Value;
use service::{ListArgs, MutationsService, NewOrder, Order, QueriesService};
use tauri::State;

use crate::AppState;

use super::{Fail, SResult, Seccess};

#[tauri::command]
pub async fn list_orders(state: State<'_, AppState>, args: ListArgs) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::list_orders(&state.db_conn, args).await;
    match res {
        Ok(res) => Ok(Seccess {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
pub async fn list_order_products(state: State<'_, AppState>, id: String) -> SResult<Vec<Value>> {
    let _ = state.db_conn;
    let res = QueriesService::list_order_products(&state.db_conn, id).await;
    match res {
        Ok(res) => Ok(Seccess {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
pub async fn create_order(state: State<'_, AppState>, order: NewOrder) -> SResult<String> {
    let _ = state.db_conn;
    let res = MutationsService::create_order(&state.db_conn, order).await;
    match res {
        Ok(id) => Ok(Seccess {
            error: None,
            message: None,
            data: Some(id),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
pub async fn update_order(state: State<'_, AppState>, order: Order) -> SResult<()> {
    let _ = state.db_conn;
    let res = MutationsService::update_order(&state.db_conn, order).await;
    match res {
        Ok(_) => Ok(Seccess {
            error: None,
            message: Option::Some(String::from("update orders success")),
            data: None,
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
pub async fn delete_order(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let _ = state.db_conn;
    let res = MutationsService::delete_order(&state.db_conn, id).await;
    match res {
        Ok(res) => Ok(Seccess {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
pub async fn get_order(state: State<'_, AppState>, id: String) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::get_order(&state.db_conn, id).await;
    match res {
        Ok(res) => Ok(Seccess {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
pub async fn get_order_details(state: State<'_, AppState>, id: String) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::get_order_details(&state.db_conn, id).await;
    match res {
        Ok(res) => Ok(Seccess {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            println!("Error: {}", err);
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}
