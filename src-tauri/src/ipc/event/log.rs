use chrono::Local;
use log::Record;
use serde::{Deserialize, Serialize};

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
}

impl LogEvent {
    pub fn from_record(record: &Record) -> Self {
        let msg = record.args().as_str().expect("Failed to get log message");
        Self {
            level: record.level().to_string(),
            module: record.module_path().map(ToString::to_string),
            target: record.target().to_string(),
            timestamp: Local::now().timestamp(),
            message: msg.to_string(),
        }
    }
}
