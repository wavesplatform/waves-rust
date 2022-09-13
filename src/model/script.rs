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
