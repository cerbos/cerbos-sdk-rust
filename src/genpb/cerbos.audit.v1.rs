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
    /// Deprecated. Use method.check_resources.inputs instead.
    #[prost(message, repeated, tag="4")]
    pub inputs: ::prost::alloc::vec::Vec<super::super::engine::v1::CheckInput>,
    /// Deprecated. Use method.check_resources.outputs instead.
    #[prost(message, repeated, tag="5")]
    pub outputs: ::prost::alloc::vec::Vec<super::super::engine::v1::CheckOutput>,
    /// Deprecated. Use method.check_resources.error instead.
    #[prost(string, tag="6")]
    pub error: ::prost::alloc::string::String,
    #[prost(oneof="decision_log_entry::Method", tags="7, 8")]
    pub method: ::core::option::Option<decision_log_entry::Method>,
}
/// Nested message and enum types in `DecisionLogEntry`.
pub mod decision_log_entry {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CheckResources {
        #[prost(message, repeated, tag="1")]
        pub inputs: ::prost::alloc::vec::Vec<super::super::super::engine::v1::CheckInput>,
        #[prost(message, repeated, tag="2")]
        pub outputs: ::prost::alloc::vec::Vec<super::super::super::engine::v1::CheckOutput>,
        #[prost(string, tag="3")]
        pub error: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlanResources {
        #[prost(message, optional, tag="1")]
        pub input: ::core::option::Option<super::super::super::engine::v1::PlanResourcesInput>,
        #[prost(message, optional, tag="2")]
        pub output: ::core::option::Option<super::super::super::engine::v1::PlanResourcesOutput>,
        #[prost(string, tag="3")]
        pub error: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="7")]
        CheckResources(CheckResources),
        #[prost(message, tag="8")]
        PlanResources(PlanResources),
    }
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
