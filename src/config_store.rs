use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Seek, Write},
};

use crate::config::Config;

#[derive(Debug)]
pub enum ConfigStoreError {
    Io(std::io::Error),
    InvalidFormat,
}

impl From<std::io::Error> for ConfigStoreError {
    fn from(e: std::io::Error) -> Self {
        ConfigStoreError::Io(e)
    }
}

pub struct ConfigStore {}

impl ConfigStore {
    pub fn load(path: &str) -> Result<Config, ConfigStoreError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let mut file = File::create_new(path)?;
                let config = &Config::new();
                file.write_all(Self::to_str(config).as_bytes())?;
                file.sync_all()?;
                file
            }
            Err(e) => Err(e)?,
        };

        file.seek(std::io::SeekFrom::Start(0))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::InvalidData => ConfigStoreError::InvalidFormat,
                _ => e.into(),
            })?;

        Ok(Self::from_str(&content)?)
    }

    pub fn persist(path: &str, config: &Config) -> Result<(), ConfigStoreError> {
        let value = Self::to_str(config);
        match OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(path)
        {
            Ok(mut file) => {
                file.write_all(value.as_bytes())?;
                file.sync_all()?;
                Ok(())
            }
            Err(e) => Err(e)?,
        }
    }

    fn from_str(content: &str) -> Result<Config, ConfigStoreError> {
        let parts: Vec<&str> = content
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect();

        if parts.len() != 2 {
            return Err(ConfigStoreError::InvalidFormat);
        }

        let mut config = Config::new();

        for part in parts {
            let (key, value) = part
                .trim()
                .split_once(':')
                .ok_or(ConfigStoreError::InvalidFormat)?;

            let cleaned_key = key.trim().trim_matches('"');

            match cleaned_key {
                "active_storage" => {
                    config.set_active_storage(value.trim().trim_matches('"'));
                }
                "storage_list" => {
                    let inner = value
                        .trim()
                        .strip_prefix('[')
                        .and_then(|s| s.strip_suffix(']'))
                        .ok_or(ConfigStoreError::InvalidFormat)?;

                    let storage_list = if inner.is_empty() {
                        Vec::new()
                    } else {
                        inner
                            .split(',')
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .collect()
                    };

                    config.set_storage_list(storage_list);
                }
                _ => return Err(ConfigStoreError::InvalidFormat),
            };
        }

        Ok(config)
    }

    fn to_str(config: &Config) -> String {
        let mut result = String::new();
        let active_storage = match config.get_active_storage() {
            Some(storage) => storage,
            _ => String::new()
        };

        result.push_str(&format!("active_storage: \"{}\"\n", active_storage));
        let storage_list = config
            .get_storage_list()
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<String>>()
            .join(",");
        result.push_str(&format!("storage_list: [{storage_list}]"));
        result
    }
}
