use prost_types::{value::Kind, Value};
use prost_types::{ListValue, Struct};

pub trait AttrVal: Sized {
    fn to_value(self) -> Value;
}

pub struct NullVal;

impl AttrVal for NullVal {
    fn to_value(self) -> Value {
        Value {
            kind: Some(Kind::NullValue(0)),
        }
    }
}

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

pub trait Attribute: Sized {
    fn to_tuple(self) -> (String, Value);
}

#[derive(Debug)]
pub struct Attr<K, V>(pub K, pub V)
where
    K: Into<String>,
    V: AttrVal;

impl<K, V> Attribute for Attr<K, V>
where
    K: Into<String>,
    V: AttrVal,
{
    fn to_tuple(self) -> (String, Value) {
        (self.0.into(), self.1.to_value())
    }
}
