use anyhow::{Context, Result};

use crate::{
    model::Session,
    utils::{JsonStorageLock, POOL},
};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Session> = JsonStorageLock::new("session");

macro_rules! session_from_record {
    ($record: expr) => {
        Self {
            id: $record.id as u32,
            fk_selected_server_id: $record.fk_selected_server_id as u32,
            all_topics: $record.all_topics,
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
        log::info!("find server by id");
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
        let id = session.id;
        let fk_selected_server_id = session.fk_selected_server_id;
        let all_topics = session.all_topics;
        let connected = session.connected;
        let pool = POOL.get().await;
        log::info!("adding server {name}");
        let record = query!(
            r#"
            INSERT INTO Session (id, fk_selected_server_id, all_topics, connected)
            VALUES (?, ?, ?, ?, ?)
            RETURNING *;
            "#,
            id,
            fk_selected_server_id,
            all_topics,
            connected
        )
        .fetch_one(&*pool)
        .await?;
        Ok(session_from_record!(record))
    }
    pub async fn select_server(server_id: u32) -> Result<()> {
        log::info!("find selected server by server_id");
        let pool = POOL.get().await;
        let sessions = query!(
            r#"
            SELECT * FROM Session
            WHERE fk_selected_server_id = ?
            "#,
            server_id
        )
        .fetch_optional(&*pool)
        .await?
        .map(|record| session_from_record!(record))
        .context(format!("Unable to retrieve selected Server by id: {}", id))?;
        Ok(sessions)
    }

    pub async fn set_listen_all_topics(enabled: bool) -> Result<()> {
        let pool = POOL.get().await;
        query!(
            r#"
            UPDATE Session
            SET all_topics = TRUE
            WHERE id = ?
            "#,
            id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }
}
