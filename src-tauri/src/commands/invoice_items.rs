use service::{InvoiceItem, MutationsService, NewInvoiceItem};
use tauri::State;

use crate::AppState;

use super::{Fail, SResult, Seccess};

#[tauri::command]
pub async fn create_invoice_item(
    state: State<'_, AppState>,
    item: NewInvoiceItem,
) -> SResult<String> {
    let _ = state.db_conn;
    let res = MutationsService::create_invoice_item(&state.db_conn, item).await;
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
pub async fn update_invoice_item(state: State<'_, AppState>, item: InvoiceItem) -> SResult<()> {
    let _ = state.db_conn;
    let res = MutationsService::update_invoice_item(&state.db_conn, item).await;
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
pub async fn delete_invoice_item(state: State<'_, AppState>, id: String) -> SResult<u64> {
    let _ = state.db_conn;
    let res = MutationsService::delete_invoice_item(&state.db_conn, id).await;
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
