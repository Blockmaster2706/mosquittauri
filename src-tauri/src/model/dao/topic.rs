use anyhow::{Context, Result};

use super::MsqtDao;
use crate::{
    model::{Session, Topic},
    utils::JsonStorageLock,
};

static STORAGE: JsonStorageLock<Topic> = JsonStorageLock::new("topic");

impl MsqtDao for Topic {
    async fn find_all() -> Result<Vec<Topic>> {
        STORAGE
            .get()?
            .find_all()
            .context("Failed to get full server list")
    }
    // SELECT * FROM Topic;
}

impl Topic {
    pub async fn try_new(server_id: u32, name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        log::info!("adding topic {name}");
        let topic = Self {
            id: STORAGE.get()?.gen_id()?,
            fk_server_id: server_id,
            name,
            enabled: false,
        };
        STORAGE.get_mut()?.insert(topic.clone())?;
        Ok(topic)
    }
    /*
     * INSERT INTO Topic (id, fk_server_id, name, enabled)
     * VALUES ({get_id()}, {server_id}, {name}, 0);
     */

    pub async fn update(id: u32, name: impl Into<String>) -> Result<()> {
        let name = name.into();
        log::info!("updating topic {name}");
        STORAGE.get_mut()?.edit(id, |topic| topic.name = name)
    }
    /*
     * UPDATE Topic
     * SET name = {name}
     * WHERE id = {id};
     */

    pub async fn set_enabled(topic_id: u32, enabled: bool) -> Result<()> {
        STORAGE.get_mut()?.edit(topic_id, |topic| {
            log::info!("setting topic {} state to {}", topic.name, enabled);
            topic.enabled = enabled
        })
    }
    /*
     * UPDATE Topic
     * SET enabled = {enabled}
     * WHERE id = {id};
     */

    #[allow(dead_code)]
    pub async fn find_enabled_by_server(server_id: u32) -> Result<Vec<Topic>> {
        log::info!("getting enabled topics for server with id");
        Ok(Self::find_all()
            .await?
            .into_iter()
            .filter(|t: &Topic| t.enabled && t.server_id() == server_id)
            .collect())
    }
    /*
     * SELECT * FROM Topic
     * WHERE fk_server_id = {server_id} AND enabled = 1;
     */

    pub async fn find_by_server(server_id: u32) -> Result<Vec<Topic>> {
        log::info!("getting topics for server with id");
        Ok(Self::find_all()
            .await?
            .into_iter()
            .filter(|t: &Topic| t.server_id() == server_id)
            .collect())
    }
    /*
     * SELECT * FROM Topic
     * WHERE fk_server_id = {server_id};
     */

    #[allow(dead_code)]
    pub async fn find_by_selected_server() -> Result<Option<Vec<Topic>>> {
        log::info!("getting topics for selected server");
        let Some(selected_server_id) = Session::get_or_init().await?.server_id() else {
            return Ok(None);
        };
        Self::find_by_server(selected_server_id).await.map(Some)
    }

    pub async fn find_enabled_by_selected_server() -> Result<Option<Vec<Topic>>> {
        log::info!("getting topics for selected server");
        let Some(selected_server_id) = Session::get_or_init().await?.server_id() else {
            return Ok(None);
        };
        Self::find_enabled_by_server(selected_server_id)
            .await
            .map(Some)
    }

    pub fn delete_by_server(server_id: u32) -> Result<()> {
        STORAGE.get_mut()?.update(|topics| {
            topics.retain(|topic| topic.server_id() != server_id);
            Ok(())
        })?;
        todo!()
    }
    /*
     * DELETE FROM Topic
     * WHERE fk_server_id = {server_id};
     */

    pub async fn delete(id: u32) -> Result<()> {
        log::info!("deleting topic with id {id}");
        STORAGE.get_mut()?.delete(id)
    }
    /*
     * DELETE FROM Topic
     * WHERE id = {id};
     */
}
