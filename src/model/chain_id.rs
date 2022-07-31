pub enum ChainId {
    MAINNET,
    TESTNET,
    STAGENET
}

impl ChainId {
    pub fn byte(&self) -> u8 {
        match *self {
            ChainId::MAINNET => 'W' as u8,
            ChainId::TESTNET => 'T' as u8,
            ChainId::STAGENET => 'S' as u8,
        }
    }
}