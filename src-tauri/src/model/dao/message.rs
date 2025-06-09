use anyhow::Result;
use chrono::Local;

use crate::{model::Message, utils::JsonStorageLock};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Message> = JsonStorageLock::new("message");

impl MsqtDao for Message {
    async fn find_all() -> Result<Vec<Self>> {
        STORAGE.get()?.find_all()
    }
    async fn find_by_id(id: u32) -> Result<Self> {
        todo!()
    }
    /*
     * SELECT * FROM Messages;
     */
}

impl Message {
    pub async fn try_new(server_id: u32, topic: String, payload: String) -> Result<Self> {
        Ok(Self {
            id: STORAGE.get()?.gen_id()?,
            fk_server_id: server_id,
            topic,
            payload,
            timestamp: Local::now().timestamp(),
        })
    }
}
/*
 * INSERT INTO Messages (id, fk_server_id, topic, payload, timestamp)
 * VALUES ({gen_id()}, {server_id}, {topic}, {payload}, {Local::now().timestamp()});
*/
