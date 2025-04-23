use anyhow::anyhow;

pub mod server;
pub mod topic;

pub trait MsqtDto {
    fn id(&self) -> Option<u64>;
    fn init_id(&mut self, id: u64) -> anyhow::Result<()>;
    fn check_id_initialized(&self) -> anyhow::Result<()> {
        if self.id().is_some() {
            return Err(anyhow!("ID for existing object cannot be changed"));
        }
        Ok(())
    }
}
