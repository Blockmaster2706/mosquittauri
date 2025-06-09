use anyhow::{Context, Result};
use rumqttc::Publish;
use serde::{Deserialize, Serialize};

use crate::model::Session;

use super::MsqtDto;

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub(in crate::model) id: u32,
    pub(in crate::model) fk_server_id: u32,
    pub(in crate::model) topic: String,
    pub(in crate::model) payload: String,
    pub(in crate::model) timestamp: i64,
}

impl MsqtDto for Message {
    fn id(&self) -> u32 {
        self.id
    }
}

#[allow(dead_code)]
impl Message {
    pub async fn try_from_publish(publish: Publish) -> Result<Self> {
        let server_id = Session::get_or_init()
            .await?
            .server_id()
            .context("No server selected")?;
        let topic = publish.topic;
        let payload =
            String::from_utf8(publish.payload.to_vec()).context("Failed to parse mqtt payload")?;
        Message::try_new(server_id, topic, payload).await
    }
    pub fn payload(&self) -> &str {
        &self.payload
    }
}
