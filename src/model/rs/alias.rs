use crate::error::{Error, Result};
use crate::model::Alias;
use crate::util::JsonDeserializer;
use serde_json::Value;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AliasesByAddressResponse {
    aliases: Vec<Alias>,
}

impl AliasesByAddressResponse {
    pub fn new(aliases: Vec<Alias>) -> Self {
        Self { aliases }
    }

    pub fn aliases(&self) -> Vec<Alias> {
        self.aliases.clone()
    }
}

impl TryFrom<&Value> for AliasesByAddressResponse {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let vec: Vec<String> = JsonDeserializer::safe_to_array(value)?
            .iter()
            .filter_map(|alias| alias.as_str())
            .map(|alias| alias.to_owned())
            .collect();

        return if vec.is_empty() {
            Ok(AliasesByAddressResponse { aliases: vec![] })
        } else {
            let chain_id = Alias::chain_id(vec[0].as_str().to_owned());
            let aliases = vec
                .iter()
                .map(|alias| Alias::new(chain_id, &alias))
                .collect::<Result<Vec<Alias>>>()?;
            Ok(AliasesByAddressResponse { aliases })
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::model::AliasesByAddressResponse;
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    pub fn aliases_by_address_response() {
        let data = fs::read_to_string("./tests/resources/alias/aliases_by_address_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let aliases_by_address: AliasesByAddressResponse = json.borrow().try_into().unwrap();

        let aliases = aliases_by_address.aliases();
        assert_eq!(aliases.len(), 3);

        let alias1 = &aliases[0];
        assert_eq!("alias:T:alias1662650000377", alias1.full_name());
        assert_eq!("alias1662650000377", alias1.name());
        let alias2 = &aliases[1];
        assert_eq!("alias:T:alias2662650000377", alias2.full_name());
        assert_eq!("alias2662650000377", alias2.name());
        let alias3 = &aliases[2];
        assert_eq!("alias:T:alias3662650000377", alias3.full_name());
        assert_eq!("alias3662650000377", alias3.name());
    }
}
