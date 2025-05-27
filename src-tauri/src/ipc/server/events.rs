use serde::Serialize;
use tauri::ipc::Channel;

use crate::model::{MsqtDao, Server};

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
#[allow(dead_code)]
pub enum ServerEvent {
    Update { list: Vec<Server> },
    Selected { id: u64 },
    Error { msg: String },
}

pub fn send_error(on_event: &Channel<ServerEvent>, msg: &impl ToString) {
    if let Err(e) = on_event.send(ServerEvent::Error {
        msg: msg.to_string(),
    }) {
        log::error!("Failed to send Server Error Event: {e:?}");
    }
}

pub fn update_servers(on_event: &Channel<ServerEvent>) -> tauri::Result<()> {
    let servers = match Server::find_all() {
        Ok(servers) => servers,
        Err(e) => {
            log::error!("Failed to get all servers {e}");
            send_error(on_event, &e);
            return Err(e.into());
        }
    };
    on_event
        .send(ServerEvent::Update { list: servers })
        .inspect_err(|e| log::error!("Failed to send Server Update event {e}"))?;
    Ok(())
}
