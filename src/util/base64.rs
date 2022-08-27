use base64::DecodeError;

pub struct Base64;

impl Base64 {
    pub fn decode(source: &str) -> Result<Vec<u8>, DecodeError> {
        let base64str = if source.starts_with("base64:") {
            source.replace("base64:", "")
        } else {
            source.to_owned()
        };
        base64::decode(base64str)
    }

    pub fn encode(bytes: &Vec<u8>, with_prefix: bool) -> String {
        let base64string = base64::encode(bytes).as_str().to_owned();
        if with_prefix {
            return format!("base64:{}", base64string);
        }
        base64string
    }
}
