pub struct AssetId {
    bytes: Vec<u8>,
}

impl AssetId {
    pub fn from_string() {

    }

    pub fn from_bytes(bytes: Vec<u8>) -> AssetId {
        AssetId { bytes }
    }

    pub fn is_waves(&self) -> bool {
        false
    }
}