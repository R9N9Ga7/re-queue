use crate::{config::Config, config_store::ConfigStore, storage::Storage};

pub struct StorageManager {
    storage: Option<Storage>,
    config: Config,
}

impl StorageManager {
    pub fn new() -> Self {
        let config = ConfigStore::load(Self::CONFIG_PATH).unwrap();
        let storage: Option<Storage> = if config.has_storages() {
            Some(Self::create_or_open_storage(&config.get_active_storage().unwrap()))
        } else {
            None
        };

        Self {
            storage,
            config,
        }
    }

    pub fn create(&mut self, storage_name: &str) {
        Self::create_or_open_storage(storage_name);

        let _ = self.config.set_active_storage(storage_name);
        let _ = self.config.add_storage(storage_name);

        self.persist();
    }

    pub fn open(&mut self, storage_name: &str) {
        self.storage = Some(Self::create_or_open_storage(storage_name));
        let _ = self.config.set_active_storage(storage_name);
        self.persist();
    }

    pub fn get_active_storage(&mut self) -> &mut Storage {
        self.storage.as_mut().unwrap()
    }

    pub fn get_active_storage_name(&mut self) -> String {
        self.config.get_active_storage().unwrap()
    }

    pub fn get_list(&mut self) -> &[String] {
        &self.config.get_storage_list()
    }

    pub fn has_storages(&mut self) -> bool {
        !self.config.get_storage_list().is_empty()
    }

    fn create_or_open_storage(storage_name: &str) -> Storage {
        Storage::new(
            Self::STORAGE_DIR_PATH,
            format!("{storage_name}.mt").as_str(),
            format!("{storage_name}.dt").as_str(),
        ).unwrap()
    }

    fn persist(&self) {
        ConfigStore::persist(Self::CONFIG_PATH, &self.config).unwrap()
    }

    const STORAGE_DIR_PATH: &str = "storage";
    const CONFIG_PATH: &str = "config";
}
