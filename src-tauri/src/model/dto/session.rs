use serde::{Deserialize, Serialize};

use super::MsqtDto;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Session {
    pub(in crate::model) id: u64,
    pub(in crate::model) fk_selected_server_id: Option<u64>,
    pub(in crate::model) connected: bool,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            id: 0,
            fk_selected_server_id: None,
            connected: true,
        }
    }
}

impl MsqtDto for Session {
    fn id(&self) -> u64 {
        self.id
    }
}

#[allow(dead_code)]
impl Session {
    fn server_id(&self) -> Option<u64> {
        self.fk_selected_server_id
    }
}
