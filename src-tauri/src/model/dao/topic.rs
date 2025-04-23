use crate::{model::Topic, utils::JsonStorage};
use anyhow::{Context, Result};

use super::MsqtDao;
impl MsqtDao for Topic {
    fn find_all() -> Result<Vec<Topic>> {
        JsonStorage::try_new("topic")?
            .find_all()
            .context("Failed to get full server list")
    }
}

impl Topic {
    pub fn find_by_server(server_id: u64) -> Result<Vec<Topic>> {
        Ok(Self::find_all()?
            .into_iter()
            .filter(|t| t.server_id() == server_id)
            .collect())
    }
}
