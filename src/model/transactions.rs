pub enum Type {
    Issue = 3,
    Transfer = 4,
    Reissue = 5,
    Burn = 6,
    Lease = 8,
    LeaseCancel = 9,
    Alias = 10,
    MassTransfer = 11,
    Data = 12,
    SetScript = 13,
    Sponsor = 14,
    SetAssetScript = 15,
}

pub enum Version {
    V1 = 1,
    V2 = 2,
}

pub type Id = Hash;
pub type Asset = Hash;

const HASH_LENGTH: usize = 32;

pub struct Hash([u8; HASH_LENGTH]);