use crate::error::{Error, Result};
use crate::model::Address;
use crate::util::JsonDeserializer;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AssetDistribution {
    items: HashMap<Address, u64>,
    last_item: Address,
    has_next: bool,
}

impl AssetDistribution {
    pub fn new(items: HashMap<Address, u64>, last_item: Address, has_next: bool) -> Self {
        Self {
            items,
            last_item,
            has_next,
        }
    }

    pub fn items(&self) -> HashMap<Address, u64> {
        self.items.clone()
    }

    pub fn last_item(&self) -> Address {
        self.last_item.clone()
    }

    pub fn has_next(&self) -> bool {
        self.has_next
    }
}

impl TryFrom<&Value> for AssetDistribution {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let has_next = JsonDeserializer::safe_to_boolean_from_field(value, "hasNext")?;
        let last_item = JsonDeserializer::safe_to_string_from_field(value, "lastItem")?;
        let items = JsonDeserializer::safe_to_map_from_field(value, "items")?
            .into_iter()
            .map(|entry| {
                Ok((
                    Address::from_string(&entry.0)?,
                    JsonDeserializer::safe_to_int(&entry.1)? as u64,
                ))
            })
            .collect::<Result<HashMap<Address, u64>>>()?;
        Ok(AssetDistribution {
            items,
            last_item: Address::from_string(&last_item)?,
            has_next,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::asset::asset_distribution::AssetDistribution;
    use crate::model::{Address, ByteString};
    use serde_json::Value;
    use std::fs;

    #[test]
    fn test_json_to_asset_distribution() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/assets/asset_distribution_rs.json")
            .expect("Unable to read file");
        let json: &Value = &serde_json::from_str(&data).expect("failed to convert");

        let asset_distribution: AssetDistribution = json.try_into()?;

        assert_eq!(true, asset_distribution.has_next());
        assert_eq!(
            "3PDh2FtgoL8ZMdbu3Zs5tCk6HuqE674SWnn",
            asset_distribution.last_item().encoded()
        );
        let items = asset_distribution.items();
        assert_eq!(
            700,
            items[&Address::from_string("3PR6Z2LHA6CBp4z2qEfALfExYvH8xaX9UhH")?]
        );
        assert_eq!(
            101500,
            items[&Address::from_string("3PDh2FtgoL8ZMdbu3Zs5tCk6HuqE674SWnn")?]
        );
        assert_eq!(
            10000,
            items[&Address::from_string("3P4U4DfUQqkfKDz9whSikqpvC9oKtWbsPtu")?]
        );
        Ok(())
    }
}
