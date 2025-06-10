use anyhow::{Context, Result};
use chrono::Local;
use sqlx::query;

use crate::{model::Message, utils::POOL};

use super::MsqtDao;

macro_rules! message_from_record {
    ($record: expr) => {
        Self {
            id: $record.id as u32,
            fk_server_id: $record.fk_server_id as u32,
            topic: $record.topic,
            payload: $record.payload,
            timestamp: $record.timestamp,
        }
    };
}

impl MsqtDao for Message {
    async fn find_all() -> Result<Vec<Self>> {
        log::info!("getting all messages");
        let pool = POOL.get().await;
        let messages = query!(
            r#"
            SELECT * FROM Message
            "#,
        )
        .fetch_all(&*pool)
        .await?
        .into_iter()
        .map(|record| message_from_record!(record))
        .collect();
        Ok(messages)
    }
    async fn find_by_id(id: u32) -> Result<Self> {
        log::info!("find message by id");
        let pool = POOL.get().await;
        let messages = query!(
            r#"
            SELECT * FROM Message
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await?
        .map(|record| message_from_record!(record))
        .context(format!("Unable to retrieve Message by id: {}", id))?;
        Ok(messages)
    }
}

impl Message {
    pub async fn try_new(server_id: u32, topic: String, payload: String) -> Result<Self> {
        let timestamp = Local::now().timestamp();
        let pool = POOL.get().await;
        log::debug!("adding message with topic {topic}");
        let record = query!(
            r#"
            INSERT INTO Message (fk_server_id, topic, payload, timestamp)
            VALUES (?, ?, ?, ?)
            RETURNING *;
            "#,
            server_id,
            topic,
            payload,
            timestamp
        )
        .fetch_one(&*pool)
        .await?;
        Ok(message_from_record!(record))
    }
}
