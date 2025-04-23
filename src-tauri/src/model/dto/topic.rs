use serde::{Deserialize, Serialize};

use super::MsqtDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    id: Option<u64>,
    fk_server_id: u64,
    name: String,
}

impl MsqtDto for Topic {
    fn id(&self) -> Option<u64> {
        self.id
    }
    fn init_id(&mut self, id: u64) -> anyhow::Result<()> {
        self.check_id_initialized()?;
        self.id = Some(id);
        Ok(())
    }
}

impl Topic {
    pub fn new(server_id: impl Into<u64>, name: impl Into<String>) -> Self {
        Self {
            id: None,
            fk_server_id: server_id.into(),
            name: name.into(),
        }
    }
    pub fn server_id(&self) -> u64 {
        self.fk_server_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
