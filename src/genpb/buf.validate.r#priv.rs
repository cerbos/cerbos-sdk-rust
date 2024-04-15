/// Do not use. Internal to protovalidate library
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldConstraints {
    #[prost(message, repeated, tag = "1")]
    pub cel: ::prost::alloc::vec::Vec<Constraint>,
}
/// Do not use. Internal to protovalidate library
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraint {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub expression: ::prost::alloc::string::String,
}
