use tauri::{ipc::Channel, Url};

use crate::model::{Server, Session};

use super::events::{send_error, update_servers, ServerEvent};

#[tauri::command]
pub async fn add_server(
    name: String,
    url: Url,
    client_id: String,
    on_event: Channel<ServerEvent>,
) -> tauri::Result<()> {
    // on_event.send()
    if let Err(e) = Server::try_new(name, url, client_id) {
        log::error!("Failed to create server: {e}");
        send_error(&on_event, &e);
    }
    update_servers(&on_event)?;
    Ok(())
}

#[tauri::command]
pub async fn selecte_server(id: u64, on_event: Channel<ServerEvent>) -> tauri::Result<()> {
    // on_event.send()
    if let Err(e) = Session::select_server(id) {
        log::error!("Failed to create server: {e}");
        send_error(&on_event, &e);
    }
    Ok(())
}

#[tauri::command]
pub async fn edit_server(
    name: String,
    url: Url,
    client_id: String,
    on_event: Channel<ServerEvent>,
) -> tauri::Result<()> {
    // on_event.send()
    Server::try_new(name, url, client_id)
        .inspect_err(|e| log::error!("Failed to create server: {e}"))?;
    update_servers(&on_event)?;
    Ok(())
}
