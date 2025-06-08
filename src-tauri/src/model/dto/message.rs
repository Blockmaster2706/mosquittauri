use anyhow::{Context, Error, Result};
use rumqttc::Publish;
use serde::{Deserialize, Serialize};

use crate::{ipc::event::MqttSendEvent, model::Session};

use super::MsqtDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub(in crate::model) id: u64,
    pub(in crate::model) fk_server_id: u64,
    pub(in crate::model) topic: String,
    pub(in crate::model) payload: String,
    pub(in crate::model) timestamp: i64,
}

impl MsqtDto for Message {
    fn id(&self) -> u64 {
        self.id
    }
}

impl TryFrom<Publish> for Message {
    type Error = Error;
    fn try_from(pkt: Publish) -> Result<Self> {
        let server_id = Session::get_or_init()?
            .server_id()
            .context("No server selected")?;
        let topic = pkt.topic;
        let payload =
            String::from_utf8(pkt.payload.to_vec()).context("Failed to parse mqtt payload")?;
        Message::try_new(server_id, topic, payload)
    }
}

#[allow(dead_code)]
impl Message {
    pub fn payload(&self) -> &str {
        &self.payload
    }
}
