use crate::model::Base64String;
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

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ArgMeta {
    arg_name: String,
    arg_type: String,
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
