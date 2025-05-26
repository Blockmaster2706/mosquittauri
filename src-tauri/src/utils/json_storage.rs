use std::{fs, marker::PhantomData, path::PathBuf};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};

use crate::model::MsqtDto;

/// Save data as json file in current dir.
///
/// Will be removed once SQLite support is done
pub struct JsonStorage<T: Serialize + DeserializeOwned + MsqtDto> {
    name: &'static str,
    storage_file: PathBuf,
    _data_type: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned + MsqtDto> JsonStorage<T> {
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

    pub fn update(&self, action: impl FnOnce(&mut Vec<T>)) -> Result<Vec<T>> {
        let mut data: Vec<T> = self.find_all()?;
        action(&mut data);
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
    pub fn insert(&self, object: T) -> Result<()> {
        self.update(|data| data.push(object))?;
        Ok(())
    }
    pub fn delete(data: &mut [T], id: u64) -> Result<()> {
        data.iter_mut().filter(|o| o.id() != id).count();
        Ok(())
    }
}
