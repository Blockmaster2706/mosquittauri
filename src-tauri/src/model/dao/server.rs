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

pub fn get_storage() -> Result<JsonStorage<Server>> {
    JsonStorage::try_new("server").context("Failed init server JsonStorage")
}

impl Server {
    pub fn try_new(
        name: impl Into<String>,
        url: impl Into<String>,
        port: u16,
        client_id: impl Into<String>,
    ) -> Result<Self> {
        let server = Self {
            name: name.into(),
            id: JsonStorage::<Server>::try_new("server")?.gen_id()?,
            url: url.into(),
            port,
            client_id: client_id.into(),
        };
        get_storage()?
            .insert(server.clone())
            .context("Failed to add server")?;
        Ok(server)
    }

    pub fn delete(id: u64) -> Result<()> {
        get_storage()?.delete(id)?;
        Ok(())
    }
}
