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
