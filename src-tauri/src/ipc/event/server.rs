use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::model::{MsqtDao, Server};

// include!("../../../gen/proto/event.server.v1.rs");

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerUpdate {
    list: Vec<Server>,
}
impl ServerUpdate {
    const ID: &str = "server-update";
    pub fn send(app: &AppHandle) -> tauri::Result<()> {
        let servers = match Server::find_all() {
            Ok(servers) => servers,
            Err(e) => {
                log::error!("Failed to get all servers {e}");
                ServerError::send(app, &e);
                return Err(e.into());
            }
        };
        app.emit(Self::ID, ServerUpdate { list: servers })
            .inspect_err(|e| log::error!("Failed to send Server Update event {e}"))?;
        Ok(())
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerSelected {
    id: u64,
}
impl ServerSelected {
    const ID: &str = "server-selected";
    pub fn send(app: &AppHandle, id: u64) {
        if let Err(e) = app.emit(Self::ID, ServerSelected { id }) {
            log::error!("Failed to send Server Error Event: {e:?}");
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerError {
    msg: String,
}
impl ServerError {
    const ID: &str = "server-error";
    pub fn send(app: &AppHandle, msg: &impl ToString) {
        if let Err(e) = app.emit(
            Self::ID,
            ServerError {
                msg: msg.to_string(),
            },
        ) {
            log::error!("Failed to send Server Error Event: {e:?}");
        }
    }
}
