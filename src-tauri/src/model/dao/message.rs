use anyhow::Result;

use crate::{model::Message, utils::JsonStorage};

use super::MsqtDao;

impl MsqtDao for Message {
    fn find_all() -> Result<Vec<Self>> {
        JsonStorage::try_new("message")?.find_all()
    }
}

impl Message {
    pub fn try_new(topic: String, payload: String) -> Result<Self> {
        Ok(Self {
            id: JsonStorage::<Message>::try_new("message")?.gen_id()?,
            topic,
            payload,
        })
    }
}
