use anyhow::Result;

use crate::{model::Message, utils::JsonStorageLock};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Message> = JsonStorageLock::new("message");

impl MsqtDao for Message {
    fn find_all() -> Result<Vec<Self>> {
        STORAGE.get()?.find_all()
    }
}

impl Message {
    pub fn try_new(topic: String, payload: String) -> Result<Self> {
        Ok(Self {
            id: STORAGE.get()?.gen_id()?,
            topic,
            payload,
        })
    }
}
