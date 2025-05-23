pub mod message;
pub mod server;
pub mod session;
pub mod topic;

pub trait MsqtDto {
    fn id(&self) -> u64;
}
