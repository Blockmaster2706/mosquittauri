use anyhow::{Context, Result};

use crate::{model::Session, utils::JsonStorageLock};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Session> = JsonStorageLock::new("session");

impl MsqtDao for Session {
    fn find_all() -> Result<Vec<Self>> {
        STORAGE.get()?.find_all()
    }
}

impl Session {
    pub fn get_or_init() -> Result<Self> {
        if let Ok(session) = Self::find_by_id(0) {
            return Ok(session);
        }
        let session = Self::default();
        STORAGE.get_mut()?.insert(session.clone())?;
        Ok(session)
    }
    pub fn select_server(server_id: u64) -> Result<()> {
        STORAGE
            .get_mut()?
            .update(|list| {
                list[0].fk_selected_server_id = Some(server_id);
                Ok(())
            })
            .context("failed to set selected server id")?;
        Ok(())
    }

    pub fn set_listen_all_topics(enabled: bool) -> Result<()> {
        STORAGE
            .get_mut()?
            .edit(0, |session| session.listen_all_topics = enabled)
    }
}
