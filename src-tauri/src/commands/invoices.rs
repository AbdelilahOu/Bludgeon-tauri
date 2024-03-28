use serde_json::Value;
use service::{Invoice, ListArgs, MutationsService, NewInvoice, QueriesService};
use tauri::State;

use crate::AppState;

use super::{Fail, SResult, Seccess};

#[tauri::command]
pub async fn list_invoices(state: State<'_, AppState>, args: ListArgs) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::list_invoices(&state.db_conn, args).await;
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
pub async fn list_invoice_products(state: State<'_, AppState>, id: String) -> SResult<Vec<Value>> {
    let _ = state.db_conn;
    let res = QueriesService::list_invoice_products(&state.db_conn, id).await;
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
pub async fn create_invoice(state: State<'_, AppState>, invoice: NewInvoice) -> SResult<String> {
    let _ = state.db_conn;
    let res = MutationsService::create_invoice(&state.db_conn, invoice).await;
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
pub async fn update_invoice(state: State<'_, AppState>, invoice: Invoice) -> SResult<()> {
    let _ = state.db_conn;
    let res = MutationsService::update_invoice(&state.db_conn, invoice).await;
    match res {
        Ok(_) => Ok(Seccess {
            error: None,
            message: Option::Some(String::from("update invoices success")),
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
pub async fn delete_invoice(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let _ = state.db_conn;
    let res = MutationsService::delete_invoice(&state.db_conn, id).await;
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
pub async fn get_invoice(state: State<'_, AppState>, id: String) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::get_invoice(&state.db_conn, id).await;
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
pub async fn get_invoice_details(state: State<'_, AppState>, id: String) -> SResult<Value> {
    let _ = state.db_conn;
    let res = QueriesService::get_invoice_details(&state.db_conn, id).await;
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
