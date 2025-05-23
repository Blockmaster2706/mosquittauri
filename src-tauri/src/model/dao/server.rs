use anyhow::{Context, Result};

use crate::{model::Server, utils::JsonStorage};

use super::MsqtDao;

impl MsqtDao for Server {
    fn find_all() -> Result<Vec<Server>> {
        JsonStorage::try_new("server")
            .context("Failed to init JsonStorage")?
            .find_all()
            .context("Failed to get full server list")
    }
}

impl Server {
    pub fn try_new(url: impl Into<String>, client_id: impl Into<String>) -> Result<Self> {
        Ok(Self {
            id: JsonStorage::<Server>::try_new("server")?.gen_id()?,
            url: url.into(),
            client_id: client_id.into(),
        })
    }
    pub fn add(server: Server) -> Result<()> {
        JsonStorage::try_new("server")?
            .update(|servers| {
                JsonStorage::insert(servers, server)
                    .err()
                    .inspect(|e| log::error!("Failed to add server {e:#?}"));
            })
            .context("Failed to add server")?;
        Ok(())
    }
}
