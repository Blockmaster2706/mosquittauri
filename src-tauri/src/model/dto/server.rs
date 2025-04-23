use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::MsqtDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    id: Option<u64>,
    url: String,
    client_id: String,
}

impl MsqtDto for Server {
    fn id(&self) -> Option<u64> {
        self.id
    }
    fn init_id(&mut self, id: u64) -> Result<()> {
        self.check_id_initialized()?;
        self.id = Some(id);
        Ok(())
    }
}

impl Server {
    pub fn new(url: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self {
            id: None,
            url: url.into(),
            client_id: client_id.into(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }
}
