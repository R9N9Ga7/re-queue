use std::{fs::{File, OpenOptions}, io::{ErrorKind, Read, Seek, Write}};

use crate::{record::Record, record_header::RecordHeader};

pub struct DataStore {
    file: File
}

impl DataStore {
    pub fn open(path: &str) -> std::io::Result<Self> {
        match OpenOptions::new().read(true).write(true).open(path) {
            Ok(file) => Ok(Self { file }),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let file = File::create_new(path)?;
                Ok(Self { file })
            },
            Err(e) => Err(e)
        }
    }

    /// Appends record and returns number of bytes written
    pub fn push(&mut self, record: &Record) -> std::io::Result<u64> {
        let bytes = record.to_bytes();
        self.file.seek(std::io::SeekFrom::End(0))?;
        self.file.write_all(&bytes)?;
        self.file.sync_data()?;

        Ok(bytes.len() as u64)
    }

    pub fn pick(&mut self, pointer: u64) -> std::io::Result<Record> {
        self.file.seek(std::io::SeekFrom::Start(pointer))?;

        let mut record_header_buffer = vec![0u8; RecordHeader::size()];
        self.file.read_exact(&mut record_header_buffer)?;

        let record_header = RecordHeader::from_bytes(&record_header_buffer);

        let mut data_buffer = Vec::<u8>::with_capacity(record_header.get_content_size() as usize);
        data_buffer.resize(record_header.get_content_size() as usize, 0);
        self.file.read_exact(&mut data_buffer)?;

        Ok(Record::from_bytes(record_header, &data_buffer))
    }
}
