use crate::error::{Error, Result};
use crate::model::Base64String;
use crate::util::JsonDeserializer;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ScriptInfo {
    script: Base64String,
    complexity: u32,
    verifier_complexity: u32,
    callable_complexities: HashMap<String, u32>,
    extra_fee: u64,
    script_text: String,
}

impl ScriptInfo {
    pub fn new(
        script: Base64String,
        complexity: u32,
        verifier_complexity: u32,
        callable_complexities: HashMap<String, u32>,
        extra_fee: u64,
        script_text: String,
    ) -> ScriptInfo {
        ScriptInfo {
            script,
            complexity,
            verifier_complexity,
            callable_complexities,
            extra_fee,
            script_text,
        }
    }

    pub fn script(&self) -> Base64String {
        self.script.clone()
    }

    pub fn complexity(&self) -> u32 {
        self.complexity
    }

    pub fn verifier_complexity(&self) -> u32 {
        self.verifier_complexity
    }

    pub fn callable_complexities(&self) -> HashMap<String, u32> {
        self.callable_complexities.clone()
    }

    pub fn extra_fee(&self) -> u64 {
        self.extra_fee
    }

    pub fn script_text(&self) -> String {
        self.script_text.clone()
    }
}

impl TryFrom<&Value> for ScriptInfo {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let script = Base64String::from_string(
            &JsonDeserializer::safe_to_string_from_field(value, "script")
                .unwrap_or_else(|_| "".to_owned()),
        )?;
        let complexity = JsonDeserializer::safe_to_int_from_field(value, "complexity")? as u32;
        let verifier_complexity =
            JsonDeserializer::safe_to_int_from_field(value, "verifierComplexity")? as u32;
        let callable_complexities: HashMap<String, u32> =
            JsonDeserializer::safe_to_map_from_field(value, "callableComplexities")?
                .into_iter()
                .map(|entry| {
                    Ok((
                        entry.0.to_owned(),
                        JsonDeserializer::safe_to_int(&entry.1)? as u32,
                    ))
                })
                .collect::<Result<HashMap<String, u32>>>()?;
        let extra_fee = JsonDeserializer::safe_to_int_from_field(value, "extraFee")? as u64;
        let script_text = JsonDeserializer::safe_to_string_from_field(value, "scriptText")
            .unwrap_or_else(|_| "".to_owned());
        Ok(ScriptInfo::new(
            script,
            complexity,
            verifier_complexity,
            callable_complexities,
            extra_fee,
            script_text,
        ))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ScriptMeta {
    meta_version: u32,
    callable_functions: HashMap<String, Vec<ArgMeta>>,
}

impl ScriptMeta {
    pub fn new(meta_version: u32, callable_functions: HashMap<String, Vec<ArgMeta>>) -> ScriptMeta {
        ScriptMeta {
            meta_version,
            callable_functions,
        }
    }

    pub fn meta_version(&self) -> u32 {
        self.meta_version
    }

    pub fn callable_functions(&self) -> HashMap<String, Vec<ArgMeta>> {
        self.callable_functions.clone()
    }
}

impl TryFrom<&Value> for ScriptMeta {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let meta_version: u32 =
            JsonDeserializer::safe_to_string_from_field(&value["meta"], "version")?
                .parse()
                .unwrap_or(0);
        if meta_version == 0 {
            return Ok(ScriptMeta::new(meta_version, HashMap::new()));
        }
        let callable_func_types =
            JsonDeserializer::safe_to_map_from_field(&value["meta"], "callableFuncTypes")?;

        let callable_functions: HashMap<String, Vec<ArgMeta>> = callable_func_types
            .into_iter()
            .map(|entry| {
                let arg_meta = JsonDeserializer::safe_to_array(&entry.1)?
                    .iter()
                    .map(|arg| arg.try_into())
                    .collect::<Result<Vec<ArgMeta>>>()?;
                Ok((entry.0, arg_meta))
            })
            .collect::<Result<HashMap<String, Vec<ArgMeta>>>>()?;
        Ok(ScriptMeta::new(meta_version, callable_functions))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ArgMeta {
    arg_name: String,
    arg_type: String,
}

impl TryFrom<&Value> for ArgMeta {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        let arg_name = JsonDeserializer::safe_to_string_from_field(value, "name")?;
        let arg_type = JsonDeserializer::safe_to_string_from_field(value, "type")?;
        Ok(ArgMeta::new(arg_name, arg_type))
    }
}

impl ArgMeta {
    pub fn new(arg_name: String, arg_type: String) -> ArgMeta {
        ArgMeta { arg_name, arg_type }
    }

    pub fn arg_name(&self) -> String {
        self.arg_name.clone()
    }

    pub fn arg_type(&self) -> String {
        self.arg_type.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::model::{ByteString, ScriptInfo, ScriptMeta};

    use serde_json::Value;
    use std::borrow::Borrow;
    use std::fs;

    #[test]
    fn test_json_to_script_info() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/addresses/script_info_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let script_info: ScriptInfo = json.borrow().try_into()?;

        assert_eq!(
            script_info.script().encoded_with_prefix(),
            "base64:AAIFAAAAAAAAAA=="
        );
        assert_eq!(script_info.complexity(), 14);
        assert_eq!(script_info.verifier_complexity(), 0);
        assert_eq!(script_info.callable_complexities()["storeData"], 14);
        assert_eq!(script_info.extra_fee(), 0);
        assert_eq!(
            script_info.script_text(),
            "DApp(DAppMeta(2,Vector(CallableFuncSignature))"
        );
        Ok(())
    }

    #[test]
    fn test_json_to_script_meta() -> Result<()> {
        let data = fs::read_to_string("./tests/resources/addresses/script_meta_rs.json")
            .expect("Unable to read file");
        let json: Value = serde_json::from_str(&data).expect("failed to generate json from str");

        let script_meta: ScriptMeta = json.borrow().try_into()?;

        assert_eq!(script_meta.meta_version(), 2);
        let function_args_meta = &script_meta.callable_functions()["storeData"];

        assert_eq!(function_args_meta[0].arg_type, "Boolean");
        assert_eq!(function_args_meta[0].arg_name, "bool");

        assert_eq!(function_args_meta[1].arg_type, "String");
        assert_eq!(function_args_meta[1].arg_name, "string");

        assert_eq!(function_args_meta[2].arg_type, "Int");
        assert_eq!(function_args_meta[2].arg_name, "integer");

        assert_eq!(function_args_meta[3].arg_type, "ByteVector");
        assert_eq!(function_args_meta[3].arg_name, "binary");

        assert_eq!(function_args_meta[4].arg_type, "List[Int]");
        assert_eq!(function_args_meta[4].arg_name, "list");

        Ok(())
    }
}
