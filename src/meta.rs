#[derive(Clone, Copy, Debug)]
pub struct Meta {
    pub version: u64,
    pub read_pointer: u64,
    pub write_pointer: u64,
    pub total_records_added: u64,
}

impl Meta {
    pub fn new(version: u64, read_pointer: u64, write_pointer: u64, total_records_added: u64) -> Self {
        Self {
            version,
            read_pointer,
            write_pointer,
            total_records_added,
        }
    }

    pub fn default() -> Meta {
        Self::new(0, 0, 0, 0)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::with_capacity(Meta::size());

        bytes.extend_from_slice(&self.version.to_le_bytes());
        bytes.extend_from_slice(&self.read_pointer.to_le_bytes());
        bytes.extend_from_slice(&self.write_pointer.to_le_bytes());
        bytes.extend_from_slice(&self.total_records_added.to_le_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        // TODO: add error handling

        let version = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let read_pointer= u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        let write_pointer = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let total_records_added = u64::from_le_bytes(bytes[24..32].try_into().unwrap());

        Self { version, read_pointer, write_pointer, total_records_added }
    }

    pub fn is_empty(&self) -> bool { self.read_pointer == self.write_pointer }

    pub const fn size() -> usize { 4 * size_of::<u64>() }
}
