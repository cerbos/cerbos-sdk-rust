#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanResourcesRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub action: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub principal: ::core::option::Option<super::super::engine::v1::Principal>,
    #[prost(message, optional, tag = "4")]
    pub resource: ::core::option::Option<
        super::super::engine::v1::plan_resources_input::Resource,
    >,
    #[prost(message, optional, tag = "5")]
    pub aux_data: ::core::option::Option<AuxData>,
    #[prost(bool, tag = "6")]
    pub include_meta: bool,
}
/// Deprecated. See CheckResourcesRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResourceSetRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub principal: ::core::option::Option<super::super::engine::v1::Principal>,
    #[prost(message, optional, tag = "4")]
    pub resource: ::core::option::Option<ResourceSet>,
    #[prost(bool, tag = "5")]
    pub include_meta: bool,
    #[prost(message, optional, tag = "6")]
    pub aux_data: ::core::option::Option<AuxData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceSet {
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_version: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "3")]
    pub instances: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        AttributesMap,
    >,
    #[prost(string, tag = "4")]
    pub scope: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributesMap {
    #[prost(map = "string, message", tag = "1")]
    pub attr: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
}
/// Deprecated. See CheckResourcesRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResourceBatchRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub principal: ::core::option::Option<super::super::engine::v1::Principal>,
    #[prost(message, repeated, tag = "3")]
    pub resources: ::prost::alloc::vec::Vec<check_resource_batch_request::BatchEntry>,
    #[prost(message, optional, tag = "4")]
    pub aux_data: ::core::option::Option<AuxData>,
}
/// Nested message and enum types in `CheckResourceBatchRequest`.
pub mod check_resource_batch_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BatchEntry {
        #[prost(string, repeated, tag = "1")]
        pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "2")]
        pub resource: ::core::option::Option<super::super::super::engine::v1::Resource>,
    }
}
/// Structure of the request for the check resources API call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResourcesRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub include_meta: bool,
    #[prost(message, optional, tag = "3")]
    pub principal: ::core::option::Option<super::super::engine::v1::Principal>,
    #[prost(message, repeated, tag = "4")]
    pub resources: ::prost::alloc::vec::Vec<check_resources_request::ResourceEntry>,
    #[prost(message, optional, tag = "5")]
    pub aux_data: ::core::option::Option<AuxData>,
}
/// Nested message and enum types in `CheckResourcesRequest`.
pub mod check_resources_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceEntry {
        #[prost(string, repeated, tag = "1")]
        pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "2")]
        pub resource: ::core::option::Option<super::super::super::engine::v1::Resource>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuxData {
    #[prost(message, optional, tag = "1")]
    pub jwt: ::core::option::Option<aux_data::Jwt>,
}
/// Nested message and enum types in `AuxData`.
pub mod aux_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Jwt {
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub key_set_id: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundValidateRequest {
    #[prost(string, tag = "1")]
    pub playground_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub files: ::prost::alloc::vec::Vec<File>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundTestRequest {
    #[prost(string, tag = "1")]
    pub playground_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub files: ::prost::alloc::vec::Vec<File>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundEvaluateRequest {
    #[prost(string, tag = "1")]
    pub playground_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub files: ::prost::alloc::vec::Vec<File>,
    #[prost(message, optional, tag = "3")]
    pub principal: ::core::option::Option<super::super::engine::v1::Principal>,
    #[prost(message, optional, tag = "4")]
    pub resource: ::core::option::Option<super::super::engine::v1::Resource>,
    #[prost(string, repeated, tag = "5")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub aux_data: ::core::option::Option<AuxData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundProxyRequest {
    #[prost(string, tag = "1")]
    pub playground_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub files: ::prost::alloc::vec::Vec<File>,
    #[prost(oneof = "playground_proxy_request::ProxyRequest", tags = "3, 4, 5, 6")]
    pub proxy_request: ::core::option::Option<playground_proxy_request::ProxyRequest>,
}
/// Nested message and enum types in `PlaygroundProxyRequest`.
pub mod playground_proxy_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProxyRequest {
        #[prost(message, tag = "3")]
        CheckResourceSet(super::CheckResourceSetRequest),
        #[prost(message, tag = "4")]
        CheckResourceBatch(super::CheckResourceBatchRequest),
        #[prost(message, tag = "5")]
        PlanResources(super::PlanResourcesRequest),
        #[prost(message, tag = "6")]
        CheckResources(super::CheckResourcesRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrUpdatePolicyRequest {
    #[prost(message, repeated, tag = "1")]
    pub policies: ::prost::alloc::vec::Vec<super::super::policy::v1::Policy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuditLogEntriesRequest {
    #[prost(enumeration = "list_audit_log_entries_request::Kind", tag = "1")]
    pub kind: i32,
    #[prost(oneof = "list_audit_log_entries_request::Filter", tags = "2, 3, 4, 5")]
    pub filter: ::core::option::Option<list_audit_log_entries_request::Filter>,
}
/// Nested message and enum types in `ListAuditLogEntriesRequest`.
pub mod list_audit_log_entries_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeRange {
        #[prost(message, optional, tag = "1")]
        pub start: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, tag = "2")]
        pub end: ::core::option::Option<::prost_types::Timestamp>,
    }
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
    pub enum Kind {
        Unspecified = 0,
        Access = 1,
        Decision = 2,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Unspecified => "KIND_UNSPECIFIED",
                Kind::Access => "KIND_ACCESS",
                Kind::Decision => "KIND_DECISION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KIND_UNSPECIFIED" => Some(Self::Unspecified),
                "KIND_ACCESS" => Some(Self::Access),
                "KIND_DECISION" => Some(Self::Decision),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        #[prost(uint32, tag = "2")]
        Tail(u32),
        #[prost(message, tag = "3")]
        Between(TimeRange),
        #[prost(message, tag = "4")]
        Since(::prost_types::Duration),
        #[prost(string, tag = "5")]
        Lookup(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesRequest {
    #[prost(bool, tag = "1")]
    pub include_disabled: bool,
    #[prost(string, tag = "2")]
    pub name_regexp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub scope_regexp: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub version_regexp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyRequest {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisablePolicyRequest {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnablePolicyRequest {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrUpdateSchemaRequest {
    #[prost(message, repeated, tag = "1")]
    pub schemas: ::prost::alloc::vec::Vec<super::super::schema::v1::Schema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSchemaRequest {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadStoreRequest {
    #[prost(bool, tag = "1")]
    pub wait: bool,
}
