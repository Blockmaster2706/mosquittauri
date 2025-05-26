use anyhow::{Context, Result};

use crate::{model::Session, utils::JsonStorage};

use super::MsqtDao;

fn get_storage() -> Result<JsonStorage<Session>> {
    Ok(JsonStorage::try_new("session").context("Failed to init session JsonStorage")?)
}

impl MsqtDao for Session {
    fn find_all() -> Result<Vec<Self>> {
        Ok(get_storage()?.find_all()?)
    }
}

impl Session {
    pub fn select_or_init() -> Result<Self> {
        if let Ok(session) = Self::find_by_id(0) {
            return Ok(session);
        }
        let session = Self::default();
        get_storage()?.insert(session.clone());
        Ok(session)
    }
}
