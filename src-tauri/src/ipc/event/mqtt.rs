use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::model::Message;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttConnectEvent {}
impl MqttConnectEvent {
    pub const ID: &str = "mqtt-connect";
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttDisconnectEvent {}
impl MqttDisconnectEvent {
    pub const ID: &str = "mqtt-disconnect";
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttSendEvent {
    topic: String,
    payload: String,
}

#[allow(dead_code)]
impl MqttSendEvent {
    pub const ID: &str = "mqtt-send";
    pub fn new(topic: impl Into<String>, payload: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            payload: payload.into(),
        }
    }
    pub fn send(&self, app: &AppHandle) -> tauri::Result<()> {
        app.emit(Self::ID, self)
    }
    pub fn topic(&self) -> &str {
        &self.topic
    }
    pub fn payload(&self) -> &str {
        &self.payload
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttSyncEvent {
    messages: Vec<Message>,
}
impl MqttSyncEvent {
    #[allow(dead_code)]
    pub const ID: &str = "mqtt-pull";
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }
    pub fn send(&self, app: &AppHandle) -> tauri::Result<()> {
        app.emit(Self::ID, self)
    }
}
