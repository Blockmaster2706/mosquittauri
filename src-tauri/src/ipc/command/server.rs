use tauri::{AppHandle, Url};

use crate::ipc::event::{ServerError, ServerSelected, ServerUpdate};
use crate::model::{Server, Session};

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
        ServerError::new(&e).send(&app);
    }
    ServerUpdate::from_all(&app)?.send(&app)?;
    Ok(())
}

#[tauri::command]
pub async fn select_server(id: u64, app: AppHandle) -> tauri::Result<()> {
    if let Err(e) = Session::select_server(id) {
        log::error!("Failed to create server: {e}");
        ServerError::new(&e).send(&app);
    }
    ServerSelected::new(id).send(&app);
    Ok(())
}

#[tauri::command]
pub async fn edit_server(
    id: u64,
    name: String,
    url: Url,
    port: u16,
    client_id: String,
    app: AppHandle,
) -> tauri::Result<()> {
    // on_event.send()
    Server::update(id, name, url, port, client_id)
        .inspect_err(|e| log::error!("Failed to create server: {e}"))?;
    ServerUpdate::from_all(&app)?.send(&app)?;
    Ok(())
}

#[tauri::command]
pub async fn get_servers(app: AppHandle) -> tauri::Result<()> {
    ServerUpdate::from_all(&app)?.send(&app)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_server(id: u64, app: AppHandle) -> tauri::Result<()> {
    // on_event.send()
    if let Err(e) = Server::delete(id) {
        log::error!("Failed to create server: {e}");
        ServerError::new(&e).send(&app);
    }
    Ok(())
}
