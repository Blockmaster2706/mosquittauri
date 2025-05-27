use anyhow::{Context, Result};

use super::MsqtDao;
use crate::{model::Topic, utils::JsonStorage};

fn get_storage() -> Result<JsonStorage<Topic>> {
    JsonStorage::<Topic>::try_new("topic").context("Failed to init JsonStorage of topic")
}

impl MsqtDao for Topic {
    fn find_all() -> Result<Vec<Topic>> {
        get_storage()?
            .find_all()
            .context("Failed to get full server list")
    }
}

impl Topic {
    pub fn try_new(server_id: impl Into<u64>, name: impl Into<String>) -> Result<Self> {
        let topic = Self {
            id: get_storage()?.gen_id()?,
            fk_server_id: server_id.into(),
            name: name.into(),
            enabled: false,
        };
        get_storage()?.insert(topic.clone())?;
        Ok(topic)
    }

    pub fn find_enabled(server_id: u64) -> Result<Vec<Topic>> {
        Ok(Self::find_all()?
            .into_iter()
            .filter(|t: &Topic| t.enabled && t.server_id() == server_id)
            .collect())
    }

    pub fn find_by_server(server_id: u64) -> Result<Vec<Topic>> {
        Ok(Self::find_all()?
            .into_iter()
            .filter(|t: &Topic| t.server_id() == server_id)
            .collect())
    }
}
