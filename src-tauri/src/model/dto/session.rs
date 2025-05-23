use super::MsqtDto;

pub struct Session {
    pub(in crate::model) id: u64,
    pub(in crate::model) fk_selected_server_id: Option<u64>,
    pub(in crate::model) connected: bool,
}

impl MsqtDto for Session {
    fn id(&self) -> u64 {
        self.id
    }
}
