use rumqttc::SubscribeFilter;
use serde::{Deserialize, Serialize};

use super::MsqtDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub(in crate::model) id: u64,
    pub(in crate::model) fk_server_id: u64,
    pub(in crate::model) name: String,
    pub(in crate::model) enabled: bool,
}

impl MsqtDto for Topic {
    fn id(&self) -> u64 {
        self.id
    }
}

#[allow(dead_code)]
impl Topic {
    pub fn server_id(&self) -> u64 {
        self.fk_server_id
    }
    pub fn get_subscribe_filter(&self) -> SubscribeFilter {
        SubscribeFilter::new(self.name.to_string(), rumqttc::QoS::ExactlyOnce)
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
