use anyhow::{Context, Result};
use chrono::Local;
use log::{Metadata, Record};
use serde::{Deserialize, Serialize};
use tauri::Emitter;

use super::MsqtEvent;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEvent {
    level: String,
    module: Option<String>,
    target: String,
    timestamp: i64,
    message: String,
}

impl MsqtEvent for LogEvent {
    const ID: &str = "log";
    fn send(&self, app: &tauri::AppHandle) -> tauri::Result<()> {
        app.emit(Self::ID, self)
            .inspect_err(|e| eprintln!("Failed to send LogEvent: {e}"))
    }
}

impl LogEvent {
    pub fn try_from_record(record: &Record) -> Result<Self> {
        let message = format!("{}", record.args())
            .split_once(" ::: ")
            .map(|args| (args.1.to_string()))
            .context("message somehow got past fomat")?;
        Ok(Self {
            level: record.level().to_string(),
            module: record.module_path().map(ToString::to_string),
            target: record.target().to_string(),
            timestamp: Local::now().timestamp(),
            message: message.to_string(),
        })
    }
}
