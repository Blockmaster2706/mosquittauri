use anyhow::{Context, Result};

use super::MsqtDao;
use crate::{
    model::{Session, Topic},
    utils::JsonStorageLock,
};

static STORAGE: JsonStorageLock<Topic> = JsonStorageLock::new("topic");

impl MsqtDao for Topic {
    fn find_all() -> Result<Vec<Topic>> {
        STORAGE
            .get()?
            .find_all()
            .context("Failed to get full server list")
    }
}

impl Topic {
    pub fn try_new(server_id: impl Into<u64>, name: impl Into<String>) -> Result<Self> {
        let topic = Self {
            id: STORAGE.get()?.gen_id()?,
            fk_server_id: server_id.into(),
            name: name.into(),
            enabled: false,
        };
        STORAGE.get_mut()?.insert(topic.clone())?;
        Ok(topic)
    }

    pub fn update(id: u64, name: impl Into<String>) -> Result<()> {
        STORAGE
            .get_mut()?
            .edit(id, |topic| topic.name = name.into())
    }

    pub fn set_enabled(topic_id: u64, enabled: bool) -> Result<()> {
        STORAGE
            .get_mut()?
            .edit(topic_id, |topic| topic.enabled = enabled)
    }

    pub fn find_enabled_by_server(server_id: u64) -> Result<Vec<Topic>> {
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

    pub fn find_by_selected_server() -> Result<Option<Vec<Topic>>> {
        let Some(selected_server_id) = Session::get_or_init()?.server_id() else {
            return Ok(None);
        };
        Ok(Some(
            Self::find_all()?
                .into_iter()
                .filter(|t: &Topic| t.server_id() == selected_server_id)
                .collect(),
        ))
    }

    pub fn delete(id: u64) -> Result<()> {
        STORAGE.get_mut()?.delete(id)
    }
}
