use std::any::type_name;
mod id;
mod log;
mod mqtt;
mod server;
mod topic;

use serde::{de::DeserializeOwned, Serialize};
use tauri::{AppHandle, Emitter};

pub use log::*;
pub use mqtt::*;
pub use server::*;
pub use topic::*;

pub trait MsqtEvent: Clone + Serialize + DeserializeOwned {
    const ID: &str;
    fn send(&self, app: &AppHandle) -> tauri::Result<()> {
        app.emit(Self::ID, self)
            .inspect_err(|e| ::log::warn!("Failed to send {:?}: {e}", type_name::<Self>()))
    }
}
