use anyhow::{Context, Result};

use super::MsqtDto;

mod message;
mod server;
mod session;
mod topic;

pub trait MsqtDao: Sized + MsqtDto {
    fn find_all() -> Result<Vec<Self>>;
    fn find_by_id(id: u64) -> Result<Self> {
        Self::find_all()?
            .into_iter()
            .find(|d| d.id() == id)
            .context(format!("No item with id {id}"))
    }
}
