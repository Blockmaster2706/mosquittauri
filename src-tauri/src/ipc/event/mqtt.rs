use serde::Serialize;

#[allow(dead_code)]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttDisconnectEvent {}
impl MqttDisconnectEvent {
    pub const ID: &str = "mqtt-disconnect";
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MqttSendEvent {
    topic: String,
    payload: String,
}

#[allow(dead_code)]
impl MqttSendEvent {
    const ID: &str = "mqtt-send";
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
    topic: String,
    payload: String,
}
impl MqttPullEvent {
    #[allow(dead_code)]
    const ID: &str = "mqtt-pull";
}
