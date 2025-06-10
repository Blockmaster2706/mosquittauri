use rumqttc::MqttOptions;
use serde::{Deserialize, Serialize};

use super::MsqtDto;

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow, Eq, PartialEq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub(in crate::model) id: u32,
    pub(in crate::model) name: String,
    pub(in crate::model) url: String,
    pub(in crate::model) port: u16,
    pub(in crate::model) client_id: String,
}

impl MsqtDto for Server {
    fn id(&self) -> u32 {
        self.id
    }
}

#[allow(dead_code)]
impl Server {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn get_mqtt_options(&self) -> MqttOptions {
        MqttOptions::new(&self.client_id, &self.url, self.port)
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }
}
