use anyhow::{Context, Result};

use crate::{model::Server, utils::JsonStorageLock};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Server> = JsonStorageLock::new("server");

impl MsqtDao for Server {
    fn find_all() -> Result<Vec<Server>> {
        STORAGE
            .get()?
            .find_all()
            .context("Failed to get full server list")
    }
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
            id: STORAGE.get()?.gen_id()?,
            url: url.into(),
            port,
            client_id: client_id.into(),
        };
        STORAGE
            .get_mut()?
            .insert(server.clone())
            .context("Failed to add server")?;
        Ok(server)
    }

    pub fn delete(id: u64) -> Result<()> {
        STORAGE.get_mut()?.delete(id)?;
        Ok(())
    }

    pub fn update(
        id: u64,
        name: impl Into<String>,
        url: impl Into<String>,
        port: u16,
        client_id: impl Into<String>,
    ) -> Result<()> {
        STORAGE.get_mut()?.edit(id, |server| {
            server.name = name.into();
            server.url = url.into();
            server.port = port;
            server.client_id = client_id.into();
        })
    }

    #[allow(dead_code)]
    pub fn find_by_name(name: &str) -> Result<Self> {
        Self::find_all()?
            .into_iter()
            .find(|server| server.name == name)
            .context(format!("No Server named {name} found"))
    }
}
