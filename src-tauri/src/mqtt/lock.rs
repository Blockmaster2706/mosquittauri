use anyhow::{anyhow, Result};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Lock<T> {
    lock: RwLock<T>,
}
impl<'a, T> Lock<T> {
    pub fn new(val: T) -> Self {
        Self {
            lock: RwLock::new(val),
        }
    }
    pub fn with<R>(&self, f: impl FnOnce(RwLockReadGuard<T>) -> R) -> Result<R> {
        Ok(f(self
            .lock
            .read()
            .map_err(|_e| anyhow!("failed to get lock for client"))?))
    }

    pub fn with_mut<R>(&'a self, f: impl FnOnce(RwLockWriteGuard<T>) -> R) -> Result<R> {
        Ok(f(self
            .lock
            .write()
            .map_err(|_e| anyhow!("failed to get lock for client"))?))
    }
}
