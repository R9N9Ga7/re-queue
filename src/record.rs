use crate::record_header::RecordHeader;

pub struct Record {
    pub meta: RecordHeader,
    pub data: String,
}

impl Record {
    pub fn new(data: String, id: u64) -> Self {
        Self {
            meta: RecordHeader::new(data.len() as u64, id),
            data,
        }
    }

    pub fn from_bytes(meta: RecordHeader, bytes: &[u8]) -> Self {
        // TODO: add error handling

        let data = String::from_utf8(bytes.to_vec()).unwrap();
        Self {
            meta,
            data,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();

        buffer.extend_from_slice(&self.meta.to_bytes());
        buffer.extend_from_slice(&self.data.as_bytes());

        buffer
    }

    pub fn size(&self) -> u64 { self.meta.get_content_size() + RecordHeader::size() as u64 }
}
