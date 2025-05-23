use crate::model::Session;

impl Session {
    pub fn select_or_create() -> Self {
        Self { id: 0 }
    }
}
