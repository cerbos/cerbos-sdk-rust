// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationError {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(enumeration = "validation_error::Source", tag = "3")]
    pub source: i32,
}
/// Nested message and enum types in `ValidationError`.
pub mod validation_error {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Source {
        Unspecified = 0,
        Principal = 1,
        Resource = 2,
    }
    impl Source {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "SOURCE_UNSPECIFIED",
                Self::Principal => "SOURCE_PRINCIPAL",
                Self::Resource => "SOURCE_RESOURCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
                "SOURCE_PRINCIPAL" => Some(Self::Principal),
                "SOURCE_RESOURCE" => Some(Self::Resource),
                _ => None,
            }
        }
    }
}
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub definition: ::prost::alloc::vec::Vec<u8>,
}
