use serde_json::Value;
use service::QueriesService;
use tauri::State;

use crate::AppState;

use super::{Fail, SResult, Seccess};

#[tauri::command]
pub async fn list_mvm_stats(state: State<'_, AppState>) -> SResult<Vec<Value>> {
    let _ = state.db_conn;
    let res = QueriesService::list_mvm_stats(&state.db_conn).await;
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
pub async fn list_top_clients(state: State<'_, AppState>) -> SResult<Vec<Value>> {
    let _ = state.db_conn;
    let res = QueriesService::list_top_clients(&state.db_conn).await;
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
pub async fn list_top_suppliers(state: State<'_, AppState>) -> SResult<Vec<Value>> {
    let _ = state.db_conn;
    let res = QueriesService::list_top_suppliers(&state.db_conn).await;
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
pub async fn list_status_count(state: State<'_, AppState>) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::list_status_count(&state.db_conn).await;
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
pub async fn list_revenue(state: State<'_, AppState>) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::list_revenue(&state.db_conn).await;
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
pub async fn list_expenses(state: State<'_, AppState>) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::list_expenses(&state.db_conn).await;
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
