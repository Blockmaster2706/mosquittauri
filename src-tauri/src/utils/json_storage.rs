use anyhow::anyhow;
use std::{
    fs,
    marker::PhantomData,
    path::PathBuf,
    sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use anyhow::{Context, Result};

use crate::model::MsqtDto;

pub type JsonStorageRead<T> = RwLockReadGuard<'static, JsonStorage<T>>;
pub type JsonStorageWrite<T> = RwLockWriteGuard<'static, JsonStorage<T>>;

pub struct JsonStorageLock<T: MsqtDto> {
    lock: OnceLock<RwLock<JsonStorage<T>>>,
    name: &'static str,
}
impl<T: MsqtDto> JsonStorageLock<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            lock: OnceLock::new(),
            name,
        }
    }
    pub fn get(&'static self) -> Result<JsonStorageRead<T>> {
        self.lock
            .get_or_init(|| self.init_storage())
            .read()
            .map_err(|_e| anyhow!("failed to get lock for {} JsonStorage", self.name))
    }

    pub fn get_mut(&'static self) -> Result<JsonStorageWrite<T>> {
        self.lock
            .get_or_init(|| self.init_storage())
            .write()
            .map_err(|_e| anyhow!("failed to get lock for {} JsonStorage", self.name))
    }

    fn init_storage(&'static self) -> RwLock<JsonStorage<T>> {
        let storage = JsonStorage::try_new(self.name)
            .unwrap_or_else(|e| panic!("Failed to init JsonStorage for {}: {e:#?}", self.name));
        RwLock::new(storage)
    }
}

/// Save data as json file in current dir.
///
/// Will be removed once SQLite support is done
pub struct JsonStorage<T: MsqtDto> {
    name: &'static str,
    storage_file: PathBuf,
    _data_type: PhantomData<T>,
}

impl<T: MsqtDto> JsonStorage<T> {
    pub fn try_new(name: &'static str) -> Result<Self> {
        let data_dir = std::env::current_dir()
            .context("Failed to get current dir")?
            .join("msqt_data");
        if !data_dir.exists() {
            fs::create_dir(&data_dir).context("Failed to create data dir")?;
        };
        let storage_file = data_dir.join(format!("{}.json", name));
        Ok(Self {
            name,
            storage_file,
            _data_type: PhantomData,
        })
    }

    pub fn update(&mut self, action: impl FnOnce(&mut Vec<T>) -> Result<()>) -> Result<Vec<T>> {
        let mut data: Vec<T> = self.find_all()?;
        action(&mut data)?;
        let new_json = serde_json::to_string_pretty(&data)?;
        fs::write(&self.storage_file, new_json)?;
        Ok(data)
    }

    pub fn find_all(&self) -> Result<Vec<T>> {
        Ok(if self.storage_file.exists() {
            let existing_json = fs::read_to_string(&self.storage_file)?;
            serde_json::from_str(&existing_json)?
        } else {
            Vec::new()
        })
    }
    pub fn gen_id(&self) -> Result<u64> {
        Ok(Self::gen_id_from_data(&self.find_all()?))
    }
    pub fn gen_id_from_data(data: &[T]) -> u64 {
        let Some(last) = data.last() else {
            return 0;
        };
        last.id() + 1
    }
    pub fn insert(&mut self, object: T) -> Result<()> {
        self.update(|data| {
            data.push(object);
            Ok(())
        })?;
        Ok(())
    }
    pub fn edit(&mut self, id: u64, action: impl FnOnce(&mut T)) -> Result<()> {
        self.update(|data| {
            action(
                data.iter_mut()
                    .find(|obj| obj.id() == id)
                    .context(format!("No object with id {id}"))?,
            );
            Ok(())
        })?;
        Ok(())
    }
    pub fn delete(&mut self, id: u64) -> Result<()> {
        let name = self.name;
        self.update(|list| {
            let index = list
                .iter()
                .position(|obj| obj.id() == id)
                .context(format!("No {} with the id {id}", name))?;
            list.remove(index);
            Ok(())
        })?;
        Ok(())
    }
}
