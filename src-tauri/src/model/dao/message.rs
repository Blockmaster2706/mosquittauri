use anyhow::Result;
use chrono::Local;

use crate::{model::Message, utils::JsonStorageLock};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Message> = JsonStorageLock::new("message");

macro_rules! message_from_record {
    ($record: expr) => {
        Self {
            id: $record.id as u32,
            fk_server_id: $record.fk_server_id as u32,
            topic: $record.topic,
            payload: $record.payload,
        }
    };
}

impl MsqtDao for Message {
    async fn find_all() -> Result<Vec<Self>> {
        log::info!("getting all messages");
        let pool = POOL.get().await;
        let messages = query!(
            r#"
            SELECT * FROM Messages
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
            SELECT * FROM Messages
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
        log::info!("adding topic {topic}");
        let record = query!(
            r#"
            INSERT INTO Messages (fk_server_id, topic, payload, timestamp)
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
        Ok(server_from_record!(record))
    }
}
