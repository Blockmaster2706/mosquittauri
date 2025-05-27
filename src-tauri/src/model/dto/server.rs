use serde::{Deserialize, Serialize};

use super::MsqtDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Server {
    pub(in crate::model) id: u64,
    pub(in crate::model) name: String,
    pub(in crate::model) url: String,
    pub(in crate::model) client_id: String,
}

impl MsqtDto for Server {
    fn id(&self) -> u64 {
        self.id
    }
}

impl Server {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }
}
