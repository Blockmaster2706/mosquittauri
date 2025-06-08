use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::model::Message;

use super::{id, MsqtEvent};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttConnectEvent {}

impl MsqtEvent for MqttConnectEvent {
    const ID: &str = id::MQTT_CONNECT;
}

impl MqttConnectEvent {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttDisconnectRequest {}
impl MsqtEvent for MqttDisconnectRequest {
    const ID: &str = id::MQTT_DISCONNECT_REQUEST;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttDisconnectEvent {}
impl MsqtEvent for MqttDisconnectEvent {
    const ID: &str = id::MQTT_DISCONNECT;
}

#[allow(unused)]
impl MqttDisconnectEvent {
    pub const ID: &str = id::MQTT_CONNECT;
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttSendEvent {
    topic: String,
    payload: String,
}

#[allow(dead_code)]
impl MqttSendEvent {
    pub const ID: &str = id::MQTT_SEND;
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
pub struct MqttPullEvent {
    messages: Vec<Message>,
}
impl MqttPullEvent {
    #[allow(dead_code)]
    pub const ID: &str = id::MQTT_PULL;
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }
    pub fn send(&self, app: &AppHandle) -> tauri::Result<()> {
        app.emit(Self::ID, self)
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttError {
    msg: String,
}
impl MqttError {
    const ID: &str = id::MQTT_ERROR;
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
