use crate::error::Result;
use crate::model::{Address, Amount, AssetId, Base64String};
use crate::util::JsonDeserializer;
use serde_json::Value;

const TYPE: u8 = 16;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InvokeScriptTransaction {
    dapp: Address,
    function: Function,
    payment: Vec<Amount>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Function {
    name: String,
    args: Vec<Arg>,
}

impl Function {
    pub fn new(name: String, args: Vec<Arg>) -> Self {
        Function { name, args }
    }

    pub fn args(&self) -> Vec<Arg> {
        self.args.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn is_default(&self) -> bool {
        self.name == "default" && self.args.is_empty()
    }
}

impl InvokeScriptTransaction {
    pub fn from_json(value: &Value) -> Result<InvokeScriptTransaction> {
        let dapp =
            Address::from_string(&JsonDeserializer::safe_to_string_from_field(value, "dApp")?)?;
        let function = map_function(value)?;
        let payments = map_payment(value)?;

        Ok(InvokeScriptTransaction {
            dapp,
            function,
            payment: payments,
        })
    }

    pub fn new(dapp: Address, function: Function, payment: Vec<Amount>) -> Self {
        InvokeScriptTransaction {
            dapp,
            function,
            payment,
        }
    }

    pub fn tx_type() -> u8 {
        TYPE
    }

    pub fn dapp(&self) -> Address {
        self.dapp.clone()
    }

    pub fn function(&self) -> Function {
        self.function.clone()
    }

    pub fn payment(&self) -> Vec<Amount> {
        self.payment.clone()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Arg {
    Binary(Base64String),
    Boolean(bool),
    Integer(i64),
    String(String),
    List(Vec<Arg>),
}

fn map_payment(value: &Value) -> Result<Vec<Amount>> {
    JsonDeserializer::safe_to_array_from_field(value, "payment")?
        .iter()
        .map(|payment| {
            let value = JsonDeserializer::safe_to_int_from_field(payment, "amount")?;
            let asset_id = match payment["assetId"].as_str() {
                Some(asset) => Some(AssetId::from_string(asset)?),
                None => None,
            };
            Ok(Amount::new(value as u64, asset_id))
        })
        .collect::<Result<Vec<Amount>>>()
}

fn map_function(value: &Value) -> Result<Function> {
    let call = JsonDeserializer::safe_to_map_from_field(value, "call")?;
    let function_name = match call.get("function") {
        Some(func_name) => JsonDeserializer::safe_to_string(func_name)?,
        None => "".to_owned(),
    };
    let args = match call.get("args") {
        Some(args) => map_args(args)?,
        None => vec![],
    };

    Ok(Function {
        name: function_name,
        args,
    })
}

fn map_args(value: &Value) -> Result<Vec<Arg>> {
    let mut args: Vec<Arg> = vec![];
    for arg in JsonDeserializer::safe_to_array(value)? {
        let arg = match JsonDeserializer::safe_to_string_from_field(&arg, "type")?.as_str() {
            "boolean" => Arg::Boolean(JsonDeserializer::safe_to_boolean_from_field(&arg, "value")?),
            "string" => Arg::String(JsonDeserializer::safe_to_string_from_field(&arg, "value")?),
            "integer" => Arg::Integer(JsonDeserializer::safe_to_int_from_field(&arg, "value")?),
            "binary" => Arg::Binary(Base64String::from_string(
                &JsonDeserializer::safe_to_string_from_field(&arg, "value")?,
            )?),
            "list" => {
                let result = map_args(&arg["value"])?;
                Arg::List(result)
            }
            _ => panic!("unknown type"),
        };
        args.push(arg);
    }
    Ok(args)
}
