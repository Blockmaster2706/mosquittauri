use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::model::{MsqtDao, Server};

use super::id;

// include!("../../../gen/proto/event.server.v1.rs");

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerUpdate {
    list: Vec<Server>,
}
impl ServerUpdate {
    const ID: &str = id::SERVER_UPDATE;
    pub fn from_all(app: &AppHandle) -> tauri::Result<Self> {
        let list = match Server::find_all() {
            Ok(list) => list,
            Err(e) => {
                log::error!("Failed to get all servers {e}");
                ServerError::new(&e).send(app);
                return Err(e.into());
            }
        };
        Ok(Self { list })
    }
    pub fn send(&self, app: &AppHandle) -> tauri::Result<()> {
        app.emit(Self::ID, self)
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
    const ID: &str = id::SERVER_SELECTED;
    pub fn new(id: u64) -> Self {
        Self { id }
    }
    pub fn send(&self, app: &AppHandle) {
        if let Err(e) = app.emit(Self::ID, self) {
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
    const ID: &str = id::SERVER_ERROR;
    pub fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
    pub fn send(&self, app: &AppHandle) {
        if let Err(e) = app.emit(Self::ID, self) {
            log::error!("Failed to send Server Error Event: {e:?}");
        }
    }
}
