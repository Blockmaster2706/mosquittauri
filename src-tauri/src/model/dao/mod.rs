use std::any::type_name;

use anyhow::{Context, Result};

use super::MsqtDto;

mod message;
mod server;
mod session;
mod topic;

pub trait MsqtDao: Sized + MsqtDto {
    fn find_all() -> Result<Vec<Self>>;
    #[allow(dead_code)]
    /// get latest version of object from database
    fn update(mut self) -> Result<Self> {
        self = Self::find_by_id(self.id())
            .context(format!("failed to update {}", type_name::<Self>()))?;
        Ok(self)
    }
    fn find_by_id(id: u64) -> Result<Self> {
        Self::find_all()?
            .into_iter()
            .find(|d| d.id() == id)
            .context(format!("No item with id {id}"))
    }
}
