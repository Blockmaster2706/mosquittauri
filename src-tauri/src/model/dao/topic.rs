use anyhow::{Context, Result};
use sqlx::query;

use super::MsqtDao;
use crate::{
    model::{Session, Topic},
    utils::{JsonStorageLock, POOL},
};

static STORAGE: JsonStorageLock<Topic> = JsonStorageLock::new("topic");

macro_rules! topic_from_record {
    ($record: expr) => {
        Self {
            id: $record.id as u32,
            fk_server_id: $record.fk_server_id as u32,
            name: $record.name,
            enabled: $record.enabled,
        }
    };
}

impl MsqtDao for Topic {
    async fn find_all() -> Result<Vec<Topic>> {
        log::info!("getting all topics");
        let pool = POOL.get().await;
        let topics = query!(
            r#"
            SELECT * FROM Topic
            "#,
        )
        .fetch_all(&*pool)
        .await?
        .into_iter()
        .map(|record| topic_from_record!(record))
        .collect();
        Ok(topics)
    }
    async fn find_by_id(id: u32) -> Result<Self> {
        log::info!("find topic by id");
        let pool = POOL.get().await;
        let topics = query!(
            r#"
            SELECT *
            FROM Topic
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await?
        .map(|record| topic_from_record!(record))
        .context(format!("Unable to retrieve Server by id: {}", id))?;
        Ok(topics)
    }
}

impl Topic {
    pub async fn try_new(server_id: u32, name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        log::info!("adding topic {name}");
        let pool = POOL.get().await;
        let record = query!(
            r#"
            INSERT INTO Topic (fk_server_id, name, enabled)
            VALUES (?, ?, 0)
            "#,
            server_id,
            name
        )
        .fetch_one(&*pool)
        .await?;
        Ok(topic_from_record!(record))
    }

    pub async fn update(id: u32, name: impl Into<String>) -> Result<()> {
        let name = name.into();
        log::info!("updating topic {name}");
        let pool = POOL.get().await;
        query!(
            r#"
            UPDATE Topic
            SET name = ?
            WHERE id = ?
            "#,
            name,
            id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }

    pub async fn set_enabled(topic_id: u32, enabled: bool) -> Result<()> {
        //log::info!(format!("setting topic with id {} state to {}", topic_id, enabled));
        let pool = POOL.get().await;
        //let enabled = enabled as i32;
        query!(
            r#"
            UPDATE Topic
            SET enabled = ?
            WHERE id = ?
            "#,
            enabled,
            topic_id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn find_enabled_by_server(server_id: u32) -> Result<Vec<Topic>> {
        log::info!("getting enabled topics for server with id");
        let pool = POOL.get().await;
        let topics = query!(
            r#"
            SELECT * FROM Topic
            WHERE fk_server_id = ? AND enabled = TRUE;
            "#,
            server_id
        )
        .fetch_optional(&*pool)
        .await?
        .map(|record| topic_from_record!(record))
        .context(format!(
            "Unable to retrieve Topics for Server with id: {}",
            server_id
        ))?;
        todo!();
        //Ok(topics)
    }

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

    pub async fn delete(id: u32) -> Result<()> {
        log::info!("deleting topic with id {id}");
        let pool = POOL.get().await;
        query!(
            r#"
            DELETE FROM Topic WHERE id = ?
            "#,
            id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }
}
