use anyhow::{Context, Result};

use crate::{model::Session, utils::JsonStorageLock};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Session> = JsonStorageLock::new("session");

impl MsqtDao for Session {
    fn find_all() -> Result<Vec<Self>> {
        STORAGE.get()?.find_all()
    }
    /*
     * SELECT * FROM Session;
    */
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
    /*
     * SELECT
     * INSERT INTO Session (id, fk_server_id, name, )
    */
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
    /*
     * SELECT * FROM Session
     * WHERE id = {id};
    */

    pub fn set_listen_all_topics(enabled: bool) -> Result<()> {
        STORAGE
            .get_mut()?
            .edit(0, |session| session.listen_all_topics = enabled)
    }
    /*
     * UPDATE Session
     * SET  all_topics = 1
     * WHERE id = {id};
    */
}
