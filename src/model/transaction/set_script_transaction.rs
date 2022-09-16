use crate::error::{Error, Result};
use crate::model::{Base64String, ByteString};
use crate::util::JsonDeserializer;
use crate::waves_proto::SetScriptTransactionData;
use serde_json::{Map, Value};

const TYPE: u8 = 13;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SetScriptTransactionInfo {
    script: Base64String,
}

impl SetScriptTransactionInfo {
    pub fn new(script: Base64String) -> Self {
        Self { script }
    }

    pub fn script(&self) -> Base64String {
        self.script.clone()
    }
}

impl TryFrom<&Value> for SetScriptTransactionInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let script = JsonDeserializer::safe_to_string_from_field(value, "script")?;

        Ok(SetScriptTransactionInfo {
            script: Base64String::from_string(&script)?,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SetScriptTransaction {
    script: Base64String,
}

impl SetScriptTransaction {
    pub fn new(script: Base64String) -> Self {
        Self { script }
    }

    pub fn script(&self) -> Base64String {
        self.script.clone()
    }

    pub fn tx_type() -> u8 {
        TYPE
    }
}

impl TryFrom<&Value> for SetScriptTransaction {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let script = JsonDeserializer::safe_to_string_from_field(value, "script")?;

        Ok(SetScriptTransaction {
            script: Base64String::from_string(&script)?,
        })
    }
}

impl TryFrom<&SetScriptTransaction> for Map<String, Value> {
    type Error = Error;

    fn try_from(value: &SetScriptTransaction) -> Result<Self> {
        let mut set_script_tx_json = Map::new();
        set_script_tx_json.insert("script".to_owned(), value.script.encoded().into());
        Ok(set_script_tx_json)
    }
}

impl TryFrom<&SetScriptTransaction> for SetScriptTransactionData {
    type Error = Error;

    fn try_from(value: &SetScriptTransaction) -> Result<Self> {
        Ok(SetScriptTransactionData {
            script: value.script.bytes(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{Base64String, ByteString, SetScriptTransaction, SetScriptTransactionInfo};
    use crate::waves_proto::SetScriptTransactionData;
    use serde_json::{json, Map, Value};
    use std::borrow::Borrow;
    use std::fs;

    const COMPILED_SCRIPT: &str = "base64:AAIFAAAAAAAAAAsIAhIHCgUCBAEIEQAAAAAAAAABAAAAA2ludgEAAAAEY2FsbAAAAAUAAAACYnYAAAABYgAAAANpbnQAAAADc3RyAAAABGxpc3QEAAAABWFzc2V0CQAEQgAAAAUCAAAABUFzc2V0AgAAAAAAAAAAAAAAAAEAAAAAAAAAAAAGBAAAAAdhc3NldElkCQAEOAAAAAEFAAAABWFzc2V0BAAAAAVsZWFzZQkABEQAAAACCAUAAAADaW52AAAABmNhbGxlcgAAAAAAAAAABwQAAAAHbGVhc2VJZAkABDkAAAABBQAAAAVsZWFzZQkABEwAAAACCQEAAAALQmluYXJ5RW50cnkAAAACAgAAAANiaW4FAAAAB2Fzc2V0SWQJAARMAAAAAgkBAAAADEJvb2xlYW5FbnRyeQAAAAICAAAABGJvb2wGCQAETAAAAAIJAQAAAAxJbnRlZ2VyRW50cnkAAAACAgAAAANpbnQAAAAAAAABiJQJAARMAAAAAgkBAAAAC1N0cmluZ0VudHJ5AAAAAgIAAAAHYXNzZXRJZAkAAlgAAAABBQAAAAdhc3NldElkCQAETAAAAAIJAQAAAAtTdHJpbmdFbnRyeQAAAAICAAAAB2xlYXNlSWQJAAJYAAAAAQUAAAAHbGVhc2VJZAkABEwAAAACCQEAAAALU3RyaW5nRW50cnkAAAACAgAAAANkZWwCAAAAAAkABEwAAAACCQEAAAALRGVsZXRlRW50cnkAAAABAgAAAANkZWwJAARMAAAAAgUAAAAFYXNzZXQJAARMAAAAAgkBAAAAClNwb25zb3JGZWUAAAACBQAAAAdhc3NldElkAAAAAAAAAAABCQAETAAAAAIJAQAAAAdSZWlzc3VlAAAAAwUAAAAHYXNzZXRJZAAAAAAAAAAABAcJAARMAAAAAgkBAAAABEJ1cm4AAAACBQAAAAdhc3NldElkAAAAAAAAAAADCQAETAAAAAIJAQAAAA5TY3JpcHRUcmFuc2ZlcgAAAAMIBQAAAANpbnYAAAAGY2FsbGVyAAAAAAAAAAACBQAAAAdhc3NldElkCQAETAAAAAIFAAAABWxlYXNlCQAETAAAAAIJAQAAAAtMZWFzZUNhbmNlbAAAAAEJAAQ5AAAAAQUAAAAFbGVhc2UFAAAAA25pbAAAAAD/oHwO";

    #[test]
    fn test_json_to_set_script_transaction() {
        let data = fs::read_to_string("./tests/resources/set_script_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let set_script_from_json: SetScriptTransactionInfo = json.borrow().try_into().unwrap();

        assert_eq!(
            set_script_from_json.script().encoded_with_prefix(),
            COMPILED_SCRIPT
        );
    }

    #[test]
    fn test_set_script_to_proto() -> Result<()> {
        let set_script_tx = &SetScriptTransaction::new(Base64String::from_string(COMPILED_SCRIPT)?);
        let proto: SetScriptTransactionData = set_script_tx.try_into()?;

        assert_eq!(proto.script, set_script_tx.script().bytes());

        Ok(())
    }

    #[test]
    fn test_set_script_to_json() -> Result<()> {
        let set_script_tx = &SetScriptTransaction::new(Base64String::from_string(COMPILED_SCRIPT)?);

        let map: Map<String, Value> = set_script_tx.try_into()?;
        let json: Value = map.into();
        let expected_json = json!({
            "script": "AAIFAAAAAAAAAAsIAhIHCgUCBAEIEQAAAAAAAAABAAAAA2ludgEAAAAEY2FsbAAAAAUAAAACYnYAAAABYgAAAANpbnQAAAADc3RyAAAABGxpc3QEAAAABWFzc2V0CQAEQgAAAAUCAAAABUFzc2V0AgAAAAAAAAAAAAAAAAEAAAAAAAAAAAAGBAAAAAdhc3NldElkCQAEOAAAAAEFAAAABWFzc2V0BAAAAAVsZWFzZQkABEQAAAACCAUAAAADaW52AAAABmNhbGxlcgAAAAAAAAAABwQAAAAHbGVhc2VJZAkABDkAAAABBQAAAAVsZWFzZQkABEwAAAACCQEAAAALQmluYXJ5RW50cnkAAAACAgAAAANiaW4FAAAAB2Fzc2V0SWQJAARMAAAAAgkBAAAADEJvb2xlYW5FbnRyeQAAAAICAAAABGJvb2wGCQAETAAAAAIJAQAAAAxJbnRlZ2VyRW50cnkAAAACAgAAAANpbnQAAAAAAAABiJQJAARMAAAAAgkBAAAAC1N0cmluZ0VudHJ5AAAAAgIAAAAHYXNzZXRJZAkAAlgAAAABBQAAAAdhc3NldElkCQAETAAAAAIJAQAAAAtTdHJpbmdFbnRyeQAAAAICAAAAB2xlYXNlSWQJAAJYAAAAAQUAAAAHbGVhc2VJZAkABEwAAAACCQEAAAALU3RyaW5nRW50cnkAAAACAgAAAANkZWwCAAAAAAkABEwAAAACCQEAAAALRGVsZXRlRW50cnkAAAABAgAAAANkZWwJAARMAAAAAgUAAAAFYXNzZXQJAARMAAAAAgkBAAAAClNwb25zb3JGZWUAAAACBQAAAAdhc3NldElkAAAAAAAAAAABCQAETAAAAAIJAQAAAAdSZWlzc3VlAAAAAwUAAAAHYXNzZXRJZAAAAAAAAAAABAcJAARMAAAAAgkBAAAABEJ1cm4AAAACBQAAAAdhc3NldElkAAAAAAAAAAADCQAETAAAAAIJAQAAAA5TY3JpcHRUcmFuc2ZlcgAAAAMIBQAAAANpbnYAAAAGY2FsbGVyAAAAAAAAAAACBQAAAAdhc3NldElkCQAETAAAAAIFAAAABWxlYXNlCQAETAAAAAIJAQAAAAtMZWFzZUNhbmNlbAAAAAEJAAQ5AAAAAQUAAAAFbGVhc2UFAAAAA25pbAAAAAD/oHwO"
        });
        assert_eq!(expected_json, json);
        Ok(())
    }
}
