#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLogEntry {
    #[prost(string, tag="1")]
    pub call_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub peer: ::core::option::Option<Peer>,
    #[prost(map="string, message", tag="4")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, MetaValues>,
    #[prost(string, tag="5")]
    pub method: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub status_code: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecisionLogEntry {
    #[prost(string, tag="1")]
    pub call_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub peer: ::core::option::Option<Peer>,
    #[prost(message, repeated, tag="4")]
    pub inputs: ::prost::alloc::vec::Vec<super::super::engine::v1::CheckInput>,
    #[prost(message, repeated, tag="5")]
    pub outputs: ::prost::alloc::vec::Vec<super::super::engine::v1::CheckOutput>,
    #[prost(string, tag="6")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaValues {
    #[prost(string, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub auth_info: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_agent: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub forwarded_for: ::prost::alloc::string::String,
}
