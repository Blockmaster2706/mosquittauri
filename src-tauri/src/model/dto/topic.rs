use serde::{Deserialize, Serialize};

use super::MsqtDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Topic {
    pub(in crate::model) id: u64,
    pub(in crate::model) fk_server_id: u64,
    pub(in crate::model) name: String,
    pub(in crate::model) enabled: bool,
}

impl MsqtDto for Topic {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Topic {
    #[allow(dead_code)]
    pub fn server_id(&self) -> u64 {
        self.fk_server_id
    }
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }
}
