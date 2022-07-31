pub enum ChainId {
    MAINNET,
    TESTNET,
    STAGENET
}

impl ChainId {
    pub fn byte(&self) -> u8 {
        match *self {
            ChainId::MAINNET => b'W',
            ChainId::TESTNET => b'T',
            ChainId::STAGENET => b'S',
        }
    }
}