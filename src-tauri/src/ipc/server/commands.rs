use tauri::{AppHandle, Url};

use crate::model::{Server, Session};

use super::events::{ServerError, ServerSelected, ServerUpdate};

#[tauri::command]
pub async fn add_server(
    name: String,
    url: Url,
    port: u16,
    client_id: String,
    app: AppHandle,
) -> tauri::Result<()> {
    // on_event.send()
    if let Err(e) = Server::try_new(name, url, port, client_id) {
        log::error!("Failed to create server: {e}");
        ServerError::send(&app, &e);
    }
    ServerUpdate::send(&app)?;
    Ok(())
}

#[tauri::command]
pub async fn get_servers(app: AppHandle) -> tauri::Result<()> {
    ServerUpdate::send(&app)?;
    Ok(())
}

#[tauri::command]
pub async fn select_server(id: u64, app: AppHandle) -> tauri::Result<()> {
    if let Err(e) = Session::select_server(id) {
        log::error!("Failed to create server: {e}");
        ServerError::send(&app, &e);
    }
    ServerSelected::send(&app, id);
    Ok(())
}

#[tauri::command]
pub async fn edit_server(
    name: String,
    url: Url,
    port: u16,
    client_id: String,
    app: AppHandle,
) -> tauri::Result<()> {
    // on_event.send()
    Server::try_new(name, url, port, client_id)
        .inspect_err(|e| log::error!("Failed to create server: {e}"))?;
    ServerUpdate::send(&app)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_server(id: u64, app: AppHandle) -> tauri::Result<()> {
    // on_event.send()
    if let Err(e) = Server::delete(id) {
        log::error!("Failed to create server: {e}");
        ServerError::send(&app, &e);
    }
    Ok(())
}
