#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_has_no_storages_and_no_active() {
        let config = Config::new();

        assert!(!config.has_storages());
        assert_eq!(config.get_active_storage(), None)
    }

    #[test]
    fn add_storage_works() {
        let mut config = Config::new();

        assert!(config.add_storage("test").is_ok());
        assert_eq!(config.get_storage_list(), &["test".to_string()]);
    }

    #[test]
    fn add_storage_duplicate_fails() {
        let mut config = Config::new();
        config.add_storage("test").unwrap();

        assert_eq!(
            config.add_storage("test"),
            Err(ConfigError::StorageAlreadyExists)
        );
    }

    #[test]
    fn set_active_storage_requires_existing_storage() {
        let mut config = Config::new();
        assert_eq!(
            config.set_active_storage("test"),
            Err(ConfigError::StorageNotFound)
        );
    }

    #[test]
    fn set_active_storage_works() {
        let mut config = Config::new();
        config.add_storage("test").unwrap();
        config.set_active_storage("test").unwrap();

        assert_eq!(config.get_active_storage(), Some("test".to_string()));
    }

    #[test]
    fn set_storage_list_rejects_duplicates() {
        let mut config = Config::new();
        let result = config.set_storage_list(vec!["test".to_string(), "test".to_string()]);
        assert!(result.is_err());
    }
}
