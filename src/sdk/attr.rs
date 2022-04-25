// Copyright 2021-2022 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
use prost_types::{value::Kind, Value};
use prost_types::{ListValue, Struct};

pub trait AttrVal: Sized {
    fn to_value(self) -> Value;
}

#[derive(Debug, Clone)]
pub struct NullVal;

impl AttrVal for NullVal {
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::NullValue(0)),
        }
    }
}

impl AttrVal for () {
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::NullValue(0)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrVal<V>(pub V)
where
    V: Into<String>;

impl<V> AttrVal for StrVal<V>
where
    V: Into<String>,
{
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::StringValue(self.0.into())),
        }
    }
}

impl AttrVal for String {
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::StringValue(self)),
        }
    }
}

impl AttrVal for &str {
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::StringValue(self.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NumVal<V>(pub V)
where
    V: Into<f64>;

impl<V> AttrVal for NumVal<V>
where
    V: Into<f64>,
{
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::NumberValue(self.0.into())),
        }
    }
}

macro_rules! impl_numeric_attr_val {
    ($t:ty) => {
        impl AttrVal for $t {
            fn to_value(self) -> Value {
                Value {
                    kind: Some(Kind::NumberValue(self as f64)),
                }
            }
        }
    };
}

impl_numeric_attr_val!(u8);
impl_numeric_attr_val!(u16);
impl_numeric_attr_val!(u32);
impl_numeric_attr_val!(u64);
impl_numeric_attr_val!(usize);

impl_numeric_attr_val!(i8);
impl_numeric_attr_val!(i16);
impl_numeric_attr_val!(i32);
impl_numeric_attr_val!(i64);
impl_numeric_attr_val!(isize);

impl_numeric_attr_val!(f32);
impl_numeric_attr_val!(f64);

#[derive(Debug, Clone)]
pub struct BoolVal<V>(pub V)
where
    V: Into<bool>;

impl<V> AttrVal for BoolVal<V>
where
    V: Into<bool>,
{
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::BoolValue(self.0.into())),
        }
    }
}

impl AttrVal for bool {
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::BoolValue(self)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructVal<K, V, I>(pub I)
where
    K: Into<String>,
    V: AttrVal,
    I: IntoIterator<Item = (K, V)>;

impl<K, V, I> AttrVal for StructVal<K, V, I>
where
    K: Into<String>,
    V: AttrVal,
    I: IntoIterator<Item = (K, V)>,
{
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::StructValue(Struct {
                fields: self
                    .0
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.to_value()))
                    .collect(),
            })),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListVal<V, I>(pub I)
where
    V: AttrVal,
    I: IntoIterator<Item = V>;

impl<V, I> AttrVal for ListVal<V, I>
where
    V: AttrVal,
    I: IntoIterator<Item = V>,
{
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::ListValue(ListValue {
                values: self.0.into_iter().map(AttrVal::to_value).collect(),
            })),
        }
    }
}

pub struct Attribute {
    key: String,
    value: Value,
}

impl Attribute {
    pub fn new<K, V>(k: K, v: V) -> Self
    where
        K: Into<String>,
        V: AttrVal,
    {
        Self {
            key: k.into(),
            value: v.to_value(),
        }
    }

    pub(crate) fn into_tuple(self) -> (String, Value) {
        (self.key, self.value)
    }
}

pub fn attr<K, V>(k: K, v: V) -> Attribute
where
    K: Into<String>,
    V: AttrVal,
{
    Attribute::new(k, v)
}

impl<K, V> From<(K, V)> for Attribute
where
    K: Into<String>,
    V: Into<String>,
{
    fn from(t: (K, V)) -> Self {
        attr(t.0.into(), t.1.into())
    }
}
