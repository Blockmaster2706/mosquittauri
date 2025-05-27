use anyhow::{Context, Result};

use crate::{model::Session, utils::JsonStorage};

use super::MsqtDao;

fn get_storage() -> Result<JsonStorage<Session>> {
    JsonStorage::try_new("session").context("Failed to init session JsonStorage")
}

impl MsqtDao for Session {
    fn find_all() -> Result<Vec<Self>> {
        get_storage()?.find_all()
    }
}

impl Session {
    pub fn get_or_init() -> Result<Self> {
        if let Ok(session) = Self::find_by_id(0) {
            return Ok(session);
        }
        let session = Self::default();
        get_storage()?.insert(session.clone())?;
        Ok(session)
    }
    pub fn select_server(server_id: u64) -> Result<()> {
        get_storage()?
            .update(|list| {
                list[0].fk_selected_server_id = Some(server_id);
                Ok(())
            })
            .context("failed to set selected server id")?;
        Ok(())
    }
}
