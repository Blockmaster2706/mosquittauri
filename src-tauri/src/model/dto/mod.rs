use serde::{de::DeserializeOwned, Serialize};

pub mod message;
pub mod server;
pub mod session;
pub mod topic;

pub trait MsqtDto: DeserializeOwned + Serialize {
    fn id(&self) -> u32;
}
