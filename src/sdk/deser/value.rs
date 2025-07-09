use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value as JsonValue;
use serde_yml::Value as YamlValue;
use std::collections::HashMap;
use std::fmt;

use crate::genpb::google::protobuf::{value::Kind, ListValue, Struct, Value};

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid protobuf Value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value {
            kind: Some(Kind::BoolValue(v)),
        })
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value {
            kind: Some(Kind::NumberValue(v as f64)),
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value {
            kind: Some(Kind::NumberValue(v as f64)),
        })
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value {
            kind: Some(Kind::NumberValue(v)),
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value {
            kind: Some(Kind::StringValue(v.to_string())),
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value {
            kind: Some(Kind::StringValue(v)),
        })
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value {
            kind: Some(Kind::NullValue(0_i32)),
        })
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value {
            kind: Some(Kind::NullValue(0_i32)),
        })
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();

        while let Some(value) = seq.next_element::<Value>()? {
            values.push(value);
        }

        Ok(Value {
            kind: Some(Kind::ListValue(ListValue { values })),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut fields = HashMap::new();

        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            fields.insert(key, value);
        }

        Ok(Value {
            kind: Some(Kind::StructValue(Struct { fields })),
        })
    }
}

impl<'de> Deserialize<'de> for Struct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StructVisitor)
    }
}

struct StructVisitor;

impl<'de> Visitor<'de> for StructVisitor {
    type Value = Struct;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map representing a protobuf Struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut fields = HashMap::new();

        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            fields.insert(key, value);
        }

        Ok(Struct { fields })
    }
}

impl<'de> Deserialize<'de> for ListValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ListValueVisitor)
    }
}

struct ListValueVisitor;

impl<'de> Visitor<'de> for ListValueVisitor {
    type Value = ListValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence representing a protobuf ListValue")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();

        while let Some(value) = seq.next_element::<Value>()? {
            values.push(value);
        }

        Ok(ListValue { values })
    }
}

// Helper functions for deserializing from JSON and YAML
pub fn from_json_str(json_str: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn from_yaml_str(yaml_str: &str) -> Result<Value, serde_yml::Error> {
    serde_yml::from_str(yaml_str)
}

// Convert from serde_json::Value to protobuf Value
pub fn from_json_value(json_value: JsonValue) -> Value {
    match json_value {
        JsonValue::Null => Value {
            kind: Some(Kind::NullValue(0_i32)),
        },
        JsonValue::Bool(b) => Value {
            kind: Some(Kind::BoolValue(b)),
        },
        JsonValue::Number(n) => Value {
            kind: Some(Kind::NumberValue(n.as_f64().unwrap_or(0.0))),
        },
        JsonValue::String(s) => Value {
            kind: Some(Kind::StringValue(s)),
        },
        JsonValue::Array(arr) => {
            let values = arr.into_iter().map(from_json_value).collect();
            Value {
                kind: Some(Kind::ListValue(ListValue { values })),
            }
        }
        JsonValue::Object(obj) => {
            let fields = obj
                .into_iter()
                .map(|(k, v)| (k, from_json_value(v)))
                .collect();
            Value {
                kind: Some(Kind::StructValue(Struct { fields })),
            }
        }
    }
}

// Convert from serde_yml::Value to protobuf Value
pub fn from_yaml_value(yaml_value: YamlValue) -> Value {
    match yaml_value {
        YamlValue::Null => Value {
            kind: Some(Kind::NullValue(0_i32)),
        },
        YamlValue::Bool(b) => Value {
            kind: Some(Kind::BoolValue(b)),
        },
        YamlValue::Number(n) => Value {
            kind: Some(Kind::NumberValue(n.as_f64().unwrap_or(0.0))),
        },
        YamlValue::String(s) => Value {
            kind: Some(Kind::StringValue(s)),
        },
        YamlValue::Sequence(seq) => {
            let values = seq.into_iter().map(from_yaml_value).collect();
            Value {
                kind: Some(Kind::ListValue(ListValue { values })),
            }
        }
        YamlValue::Mapping(map) => {
            let mut fields = HashMap::new();
            for (k, v) in map {
                let key = match k {
                    YamlValue::String(s) => s,
                    YamlValue::Number(n) => n.to_string(),
                    YamlValue::Bool(b) => b.to_string(),
                    _ => "unknown".to_string(),
                };
                fields.insert(key, from_yaml_value(v));
            }
            Value {
                kind: Some(Kind::StructValue(Struct { fields })),
            }
        }
        YamlValue::Tagged(tagged) => from_yaml_value(tagged.value),
    }
}

// Auto-detect format and deserialize
pub fn deserialize_auto(input: &str) -> Result<Value, Box<dyn std::error::Error>> {
    // Try JSON first
    if let Ok(value) = from_json_str(input) {
        return Ok(value);
    }

    // If JSON fails, try YAML
    match from_yaml_str(input) {
        Ok(value) => Ok(value),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn get_kind(value: &Value) -> Option<&Kind> {
    value.kind.as_ref()
}

pub fn is_null(value: &Value) -> bool {
    matches!(value.kind, Some(Kind::NullValue(_)))
}

pub fn as_number(value: &Value) -> Option<f64> {
    match &value.kind {
        Some(Kind::NumberValue(n)) => Some(*n),
        _ => None,
    }
}

pub fn as_string(value: &Value) -> Option<&str> {
    match &value.kind {
        Some(Kind::StringValue(s)) => Some(s),
        _ => None,
    }
}

pub fn as_bool(value: &Value) -> Option<bool> {
    match &value.kind {
        Some(Kind::BoolValue(b)) => Some(*b),
        _ => None,
    }
}

pub fn as_struct(value: &Value) -> Option<&Struct> {
    match &value.kind {
        Some(Kind::StructValue(s)) => Some(s),
        _ => None,
    }
}

pub fn as_list(value: &Value) -> Option<&ListValue> {
    match &value.kind {
        Some(Kind::ListValue(l)) => Some(l),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_deserialization() {
        let json_str = r#"
        {
            "name": "John Doe",
            "age": 30,
            "is_active": true,
            "scores": [85, 92, 78],
            "address": {
                "street": "123 Main St",
                "city": "Anytown"
            },
            "optional_field": null
        }
        "#;

        let result = from_json_str(json_str).unwrap();

        if let Some(Kind::StructValue(s)) = &result.kind {
            assert!(s.fields.contains_key("name"));
            assert!(s.fields.contains_key("age"));
            assert!(s.fields.contains_key("is_active"));
            assert!(s.fields.contains_key("scores"));
            assert!(s.fields.contains_key("address"));
            assert!(s.fields.contains_key("optional_field"));

            // Check specific values
            assert_eq!(as_string(&s.fields["name"]), Some("John Doe"));
            assert_eq!(as_number(&s.fields["age"]), Some(30.0));
            assert_eq!(as_bool(&s.fields["is_active"]), Some(true));
            assert!(is_null(&s.fields["optional_field"]));
        } else {
            panic!("Expected StructValue");
        }
    }

    #[test]
    fn test_yaml_deserialization() {
        let yaml_str = r#"
name: Jane Smith
age: 25
is_student: false
subjects:
  - Math
  - Science
  - History
contact:
  email: jane@example.com
  phone: "555-1234"
graduation_date: null
        "#;

        let result = from_yaml_str(yaml_str).unwrap();

        if let Some(Kind::StructValue(s)) = &result.kind {
            assert_eq!(as_string(&s.fields["name"]), Some("Jane Smith"));
            assert_eq!(as_number(&s.fields["age"]), Some(25.0));
            assert_eq!(as_bool(&s.fields["is_student"]), Some(false));
            assert!(is_null(&s.fields["graduation_date"]));

            // Check array
            if let Some(list) = as_list(&s.fields["subjects"]) {
                assert_eq!(list.values.len(), 3);
                assert_eq!(as_string(&list.values[0]), Some("Math"));
                assert_eq!(as_string(&list.values[1]), Some("Science"));
                assert_eq!(as_string(&list.values[2]), Some("History"));
            } else {
                panic!("Expected ListValue for subjects");
            }
        } else {
            panic!("Expected StructValue");
        }
    }

    #[test]
    fn test_auto_detection() {
        let json_input = r#"{"type": "json", "valid": true}"#;
        let yaml_input = r#"
type: yaml
valid: true
        "#;

        let json_result = deserialize_auto(json_input).unwrap();
        let yaml_result = deserialize_auto(yaml_input).unwrap();

        // Both should deserialize successfully
        assert!(matches!(json_result.kind, Some(Kind::StructValue(_))));
        assert!(matches!(yaml_result.kind, Some(Kind::StructValue(_))));
    }

    #[test]
    fn test_primitive_types() {
        let num_result = from_json_str("42.5").unwrap();
        assert_eq!(as_number(&num_result), Some(42.5));

        let str_result = from_json_str(r#""hello world""#).unwrap();
        assert_eq!(as_string(&str_result), Some("hello world"));

        let bool_result = from_json_str("true").unwrap();
        assert_eq!(as_bool(&bool_result), Some(true));

        let null_result = from_json_str("null").unwrap();
        assert!(is_null(&null_result));
    }
}
