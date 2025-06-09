use anyhow::{Context, Result};
use sqlx::query;

use crate::{
    model::Server,
    utils::{JsonStorageLock, POOL},
};

use super::MsqtDao;

static STORAGE: JsonStorageLock<Server> = JsonStorageLock::new("server");

macro_rules! server_from_record {
    ($record: expr) => {
        Self {
            id: $record.id as u32,
            name: $record.name,
            url: $record.url,
            port: $record.port as u16,
            client_id: $record.client_id,
        }
    };
}

impl MsqtDao for Server {
    async fn find_all() -> Result<Vec<Server>> {
        log::info!("getting all servers");
        let pool = POOL.get().await;
        let servers = query!(
            r#"
SELECT id, name, url, port, client_id
FROM Server
            "#,
        )
        .fetch_all(&*pool)
        .await?
        .into_iter()
        .map(|record| server_from_record!(record))
        .collect();
        Ok(servers)
    }
    /*
     * SELECT * FROM Server;
     */
}

impl Server {
    pub async fn try_new(
        name: impl Into<String>,
        url: impl Into<String>,
        port: u16,
        client_id: impl Into<String>,
    ) -> Result<Self> {
        let name = name.into();
        let url = url.into();
        let client_id = client_id.into();
        let pool = POOL.get().await;
        log::info!("adding server {name}");
        let record = query!(
            r#"
        INSERT INTO Server (name, url, port, client_id)
        VALUES (?, ?, ?, ?)
        RETURNING *;
                "#,
            name,
            url,
            port,
            client_id,
        )
        .fetch_one(&*pool)
        .await?;
        Ok(server_from_record!(record))
    }

    pub fn delete(id: u32) -> Result<()> {
        log::info!("deleting server with id {id}");
        STORAGE.get_mut()?.delete(id)?;
        Ok(())
    }
    /*
     * DELETE FROM Sever WHERE id = {id};
     */

    pub async fn update(
        id: u32,
        name: impl Into<String>,
        url: impl Into<String>,
        port: u16,
        client_id: impl Into<String>,
    ) -> Result<()> {
        let name = name.into();
        let url = url.into();
        log::info!("updating server {name}");
        STORAGE.get_mut()?.edit(id, |server| {
            server.name = name;
            server.url = url;
            server.port = port;
            server.client_id = client_id.into();
        })
    }
    /*
     * UPDATE Server
     * SET name = {name},
     * url = {url},
     * port = {port},
     * client_id = {client_id},
     * WHERE id = {id};
     */

    #[allow(dead_code)]
    pub async fn find_by_name(name: &str) -> Result<Self> {
        Self::find_all()
            .await?
            .into_iter()
            .find(|server| server.name == name)
            .context(format!("No Server named {name} found"))
    }
    /*
     * SELECT * FROM Server
     * WHERE name LIKE '%{name}%';
     */
}
