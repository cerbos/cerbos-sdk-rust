#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationError {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(enumeration="validation_error::Source", tag="3")]
    pub source: i32,
}
/// Nested message and enum types in `ValidationError`.
pub mod validation_error {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Source {
        Unspecified = 0,
        Principal = 1,
        Resource = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub definition: ::prost::alloc::vec::Vec<u8>,
}
