use std::{fs, path::Path};

use crate::{
    data_store::DataStore, meta_store::MetaStore, record::Record, record_header::RecordHeader,
};

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Empty,
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::Io(e)
    }
}

pub struct Storage {
    meta_store: MetaStore,
    data_store: DataStore,
}

impl Storage {
    pub fn new(dir_path: &str, meta_store_path: &str, data_store_path: &str) -> Result<Self, StorageError> {
        fs::create_dir_all(dir_path)?;

        let meta_store_full_path = Self::file_path(dir_path, meta_store_path);
        let data_store_full_path = Self::file_path(dir_path, data_store_path);

        Ok(Self {
            meta_store: MetaStore::open(meta_store_full_path.to_str().unwrap())?,
            data_store: DataStore::open(data_store_full_path.to_str().unwrap())?,
        })
    }

    pub fn save(&mut self, value: String) -> Result<(), StorageError> {
        let record = Record::new(value);

        let mut meta = self.meta_store.get()?;
        meta.write_pointer += self.data_store.push(&record)?;
        meta.total_records_added += 1;
        self.meta_store.update(meta)?;

        Ok(())
    }

    pub fn pick(&mut self) -> Result<String, StorageError> {
        let meta = self.meta_store.get()?;
        if meta.is_empty() {
            return Err(StorageError::Empty);
        }

        Ok(self.data_store.pick(meta.read_pointer)?.data)
    }

    pub fn move_next(&mut self) -> Result<(), StorageError> {
        let mut meta = self.meta_store.get()?;

        let record = self.data_store.pick(meta.read_pointer)?;

        let next_record_read_pointer = meta.read_pointer as usize
            + RecordHeader::size()
            + record.meta.get_content_size() as usize;

        if next_record_read_pointer < meta.write_pointer as usize {
            meta.read_pointer = next_record_read_pointer as u64;
        } else {
            meta.read_pointer = 0;
        }

        self.meta_store.update(meta)?;

        Ok(())
    }

    pub fn get_all(&mut self) -> Result<Vec<Record>, StorageError> {
        Ok(self.data_store.get_all()?)
    }

    pub fn file_path(dir_path: &str, file_path: &str) -> std::path::PathBuf {
        let base = Path::new(dir_path);
        base.join(file_path)
    }
}
