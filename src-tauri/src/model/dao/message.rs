use anyhow::Result;
use chrono::Local;

use crate::{model::Message, utils::JsonStorageLock};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Message> = JsonStorageLock::new("message");

impl MsqtDao for Message {
    fn find_all() -> Result<Vec<Self>> {
        STORAGE.get()?.find_all()
    }
    /*
     * SELECT * FROM Messages;
    */
}

impl Message {
    pub fn try_new(server_id: u64, topic: String, payload: String) -> Result<Self> {
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
