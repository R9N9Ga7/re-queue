pub struct RecordHeader {
    is_active: bool,
    content_size: u64,
    id: u64,
}

impl RecordHeader {
    pub fn new(content_size: u64, id: u64) -> Self {
        Self { is_active: true, content_size, id }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        // TODO: add error handling
        let is_active = bytes[0] != 0;
        let content_size = u64::from_le_bytes(bytes[1..9].try_into().unwrap());
        let id = u64::from_le_bytes(bytes[9..17].try_into().unwrap());

        Self {
            is_active,
            content_size,
            id,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::with_capacity(Self::size());

        buffer.extend_from_slice(&[self.is_active as u8]);
        buffer.extend_from_slice(&self.content_size.to_le_bytes());
        buffer.extend_from_slice(&self.id.to_le_bytes());

        buffer
    }

    pub fn get_content_size(&self) -> u64 { self.content_size }
    pub fn get_id(&self) -> u64 { self.id }

    pub const fn size() -> usize { 2 * size_of::<u64>() + 1 }
}
