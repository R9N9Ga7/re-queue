use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Seek, Write},
};

use crate::meta::Meta;

pub struct MetaStore {
    file: File,
    meta: Option<Meta>,
}

impl MetaStore {
    pub fn open(path: &str) -> std::io::Result<Self> {
        match OpenOptions::new().read(true).write(true).open(path) {
            Ok(file) => Ok(Self { file, meta: None }),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let mut file = File::create_new(path)?;
                file.write_all(&Meta::default().to_bytes())?;
                Ok(Self { file, meta: None })
            }
            Err(e) => Err(e),
        }
    }

    pub fn get(&mut self) -> std::io::Result<Meta> {
        if let Some(meta) = self.meta {
            return Ok(meta);
        }

        let mut buffer: [u8; Meta::size()] = [0; Meta::size()];
        self.file.seek(std::io::SeekFrom::Start(0))?;
        self.file.read_exact(&mut buffer)?;
        self.meta = Some(Meta::from_bytes(&buffer));

        Ok(self.meta.unwrap())
    }

    pub fn update(&mut self, meta: Meta) -> std::io::Result<()> {
        self.file.seek(std::io::SeekFrom::Start(0))?;
        self.file.write_all(&meta.to_bytes())?;
        self.file.sync_all()?;
        self.meta = Some(meta);

        Ok(())
    }
}
