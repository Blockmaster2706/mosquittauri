use anyhow::{Context, Result};
use sqlx::query;

use crate::{model::Session, utils::POOL};

use super::MsqtDao;

macro_rules! session_from_record {
    ($record: expr) => {
        Self {
            id: $record.id as u32,
            fk_selected_server_id: $record
                .fk_selected_server_id
                .map(|server_id| server_id as u32),
            listen_all_topics: $record.listen_all_topics,
            connected: $record.connected,
        }
    };
}

impl MsqtDao for Session {
    async fn find_all() -> Result<Vec<Self>> {
        log::info!("getting all sessions");
        let pool = POOL.get().await;
        let sessions = query!(r#"SELECT * FROM Session"#,)
            .fetch_all(&*pool)
            .await?
            .into_iter()
            .map(|record| session_from_record!(record))
            .collect();
        Ok(sessions)
    }
    async fn find_by_id(id: u32) -> Result<Self> {
        log::info!("find session by id");
        let pool = POOL.get().await;
        let sessions = query!(
            r#"
            SELECT * FROM Session WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await?
        .map(|record| session_from_record!(record))
        .context(format!("Unable to retrieve Server by id: {}", id))?;
        Ok(sessions)
    }
}

impl Session {
    pub async fn get_or_init() -> Result<Self> {
        if let Ok(session) = Self::find_by_id(0).await {
            return Ok(session);
        }
        let session = Self::default();
        let pool = POOL.get().await;
        let record = query!(
            r#"
            INSERT INTO Session (id, fk_selected_server_id, listen_all_topics, connected)
            VALUES (?, ?, ?, ?)
            RETURNING *;
            "#,
            session.id,
            session.fk_selected_server_id,
            session.listen_all_topics,
            session.connected
        )
        .fetch_one(&*pool)
        .await?;
        Ok(session_from_record!(record))
    }
    pub async fn select_server(server_id: u32) -> Result<()> {
        log::info!("set selected server by server_id");
        let pool = POOL.get().await;
        query!(
            r#"
            UPDATE Session
            SET fk_selected_server_id = ?
            WHERE id = 0
            "#,
            server_id
        )
        .execute(&*pool)
        .await
        .context(format!(
            "Unable to retrieve selected Server by id: {}",
            server_id
        ))?;
        Ok(())
    }

    pub async fn set_listen_all_topics(enabled: bool) -> Result<()> {
        let pool = POOL.get().await;
        query!(
            r#"
            UPDATE Session
            SET listen_all_topics = ?
            WHERE id = 0
            "#,
            enabled
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }
}
