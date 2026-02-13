pub enum ConfigError {
    StorageNotFound,
    StorageAlreadyExists,
}

#[derive(Debug)]
pub struct Config {
    active_storage: String,
    storage_list: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            active_storage: String::from(""),
            storage_list: Vec::new(),
        }
    }

    pub fn set_active_storage(&mut self, storage_name: &str) -> Result<(), ConfigError> {
        if !self.storage_list.contains(&storage_name.to_string()) {
            return Err(ConfigError::StorageNotFound);
        }

        self.active_storage = storage_name.to_string();

        Ok(())
    }

    pub fn get_active_storage(&self) -> Option<String> {
        if self.active_storage.is_empty() {
            None
        } else {
            Some(self.active_storage.clone())
        }
    }

    pub fn add_storage(&mut self, storage_name: &str) -> Result<(), ConfigError> {
        if self.storage_list.contains(&storage_name.to_string()) {
            return Err(ConfigError::StorageAlreadyExists);
        }

        self.storage_list.push(storage_name.to_string());

        Ok(())
    }

    pub fn set_storage_list(&mut self, storage_list: Vec<String>) -> Result<(), ConfigError> {
        for storage in storage_list {
            self.add_storage(&storage)?;
        }

        Ok(())
    }

    pub fn get_storage_list(&self) -> &[String] {
        &self.storage_list
    }

    pub fn has_storages(&self) -> bool {
        !self.storage_list.is_empty()
    }
}
