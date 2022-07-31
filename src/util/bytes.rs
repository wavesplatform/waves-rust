pub struct Bytes;

impl Bytes {
    pub fn from_nonce(number: u8) -> Vec<u8> {
        let mut bytes: [u8; 4] = [0; 4];
        bytes[3] = number;
        Vec::from(bytes)
    }

    pub fn concat(bytes: Vec<Vec<u8>>) -> Vec<u8> {
        bytes.concat()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Bytes;

    #[test]
    fn test_from_nonce() {
        let vec = Bytes::from_nonce(127);
        assert_eq!(vec, Vec::from([0, 0, 0, 127]))
    }

    #[test]
    fn test_concat() {
        let a: [u8; 4] = [0, 0, 0, 127];
        let b: [u8; 3] = [1, 2, 3];

        let bytes: Vec<Vec<u8>> = vec![a.to_vec(), b.to_vec()];
        assert_eq!(Bytes::concat(bytes), Vec::from([0, 0, 0, 127, 1, 2, 3]))
    }
}