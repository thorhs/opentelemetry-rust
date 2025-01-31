use std::{borrow::Cow, collections::BTreeMap};

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub(crate) struct AttributeSet(BTreeMap<Key, Value>);

impl From<&opentelemetry_sdk::AttributeSet> for AttributeSet {
    fn from(value: &opentelemetry_sdk::AttributeSet) -> Self {
        AttributeSet(
            value
                .iter()
                .map(|(key, value)| (Key::from(key.clone()), Value::from(value.clone())))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Resource {
    attributes: Vec<KeyValue>,
    #[serde(skip_serializing_if = "is_zero")]
    dropped_attributes_count: u64,
}

fn is_zero(v: &u64) -> bool {
    *v == 0
}

impl From<&opentelemetry_sdk::Resource> for Resource {
    fn from(value: &opentelemetry_sdk::Resource) -> Self {
        Resource {
            attributes: value
                .iter()
                .map(|(key, value)| KeyValue {
                    key: key.clone().into(),
                    value: value.clone().into(),
                })
                .collect(),
            dropped_attributes_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Key(Cow<'static, str>);

impl From<opentelemetry_api::Key> for Key {
    fn from(value: opentelemetry_api::Key) -> Self {
        Key(value.as_str().to_string().into())
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub(crate) enum Value {
    #[serde(rename = "boolValue")]
    Bool(bool),
    #[serde(rename = "intValue")]
    Int(i64),
    #[serde(rename = "doubleValue")]
    Double(f64),
    #[serde(rename = "stringValue")]
    String(String),
    #[serde(rename = "arrayValue")]
    Array(Vec<Value>),
}

impl From<opentelemetry_api::Value> for Value {
    fn from(value: opentelemetry_api::Value) -> Self {
        match value {
            opentelemetry_api::Value::Bool(b) => Value::Bool(b),
            opentelemetry_api::Value::I64(i) => Value::Int(i),
            opentelemetry_api::Value::F64(f) => Value::Double(f),
            opentelemetry_api::Value::String(s) => Value::String(s.into()),
            opentelemetry_api::Value::Array(a) => match a {
                opentelemetry_api::Array::Bool(b) => {
                    Value::Array(b.into_iter().map(Value::Bool).collect())
                }
                opentelemetry_api::Array::I64(i) => {
                    Value::Array(i.into_iter().map(Value::Int).collect())
                }
                opentelemetry_api::Array::F64(f) => {
                    Value::Array(f.into_iter().map(Value::Double).collect())
                }
                opentelemetry_api::Array::String(s) => {
                    Value::Array(s.into_iter().map(|s| Value::String(s.into())).collect())
                }
            },
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KeyValue {
    key: Key,
    value: Value,
}

impl From<opentelemetry_api::KeyValue> for KeyValue {
    fn from(value: opentelemetry_api::KeyValue) -> Self {
        KeyValue {
            key: value.key.into(),
            value: value.value.into(),
        }
    }
}

impl From<&opentelemetry_api::KeyValue> for KeyValue {
    fn from(value: &opentelemetry_api::KeyValue) -> Self {
        KeyValue {
            key: value.key.clone().into(),
            value: value.value.clone().into(),
        }
    }
}

impl From<(opentelemetry_api::Key, opentelemetry_api::Value)> for KeyValue {
    fn from((key, value): (opentelemetry_api::Key, opentelemetry_api::Value)) -> Self {
        KeyValue {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Scope {
    #[serde(skip_serializing_if = "str::is_empty")]
    name: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attributes: Vec<KeyValue>,
    #[serde(skip_serializing_if = "is_zero")]
    dropped_attributes_count: u64,
}

impl From<opentelemetry_sdk::Scope> for Scope {
    fn from(value: opentelemetry_sdk::Scope) -> Self {
        Scope {
            name: value.name,
            version: value.version,
            attributes: Vec::new(),
            dropped_attributes_count: 0,
        }
    }
}
