use serde::{Deserialize, Serialize};

use super::MsqtDto;

#[derive(Clone, Serialize, Deserialize, Debug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub(in crate::model) id: u32,
    pub(in crate::model) fk_selected_server_id: Option<u32>,
    pub(in crate::model) listen_all_topics: bool,
    pub(in crate::model) connected: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Session {
    fn default() -> Self {
        Self {
            id: 0,
            fk_selected_server_id: None,
            listen_all_topics: false,
            connected: false,
        }
    }
}

impl MsqtDto for Session {
    fn id(&self) -> u32 {
        self.id
    }
}

#[allow(dead_code)]
impl Session {
    pub fn server_id(&self) -> Option<u32> {
        self.fk_selected_server_id
    }
    pub fn listen_all_topics(&self) -> bool {
        self.listen_all_topics
    }
    pub fn connected(&self) -> bool {
        self.connected
    }
}
