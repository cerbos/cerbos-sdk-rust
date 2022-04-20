#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourcesQueryPlanResponse {
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub resource_kind: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub policy_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub filter: ::core::option::Option<resources_query_plan_response::Filter>,
    #[prost(message, optional, tag="6")]
    pub meta: ::core::option::Option<resources_query_plan_response::Meta>,
}
/// Nested message and enum types in `ResourcesQueryPlanResponse`.
pub mod resources_query_plan_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Expression {
        #[prost(string, tag="1")]
        pub operator: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="2")]
        pub operands: ::prost::alloc::vec::Vec<expression::Operand>,
    }
    /// Nested message and enum types in `Expression`.
    pub mod expression {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Operand {
            #[prost(oneof="operand::Node", tags="1, 2, 3")]
            pub node: ::core::option::Option<operand::Node>,
        }
        /// Nested message and enum types in `Operand`.
        pub mod operand {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Node {
                #[prost(message, tag="1")]
                Value(::prost_types::Value),
                #[prost(message, tag="2")]
                Expression(super::super::Expression),
                #[prost(string, tag="3")]
                Variable(::prost::alloc::string::String),
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        #[prost(enumeration="filter::Kind", tag="1")]
        pub kind: i32,
        #[prost(message, optional, tag="2")]
        pub condition: ::core::option::Option<expression::Operand>,
    }
    /// Nested message and enum types in `Filter`.
    pub mod filter {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Kind {
            Unspecified = 0,
            AlwaysAllowed = 1,
            AlwaysDenied = 2,
            Conditional = 3,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Meta {
        #[prost(string, tag="1")]
        pub filter_debug: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub matched_scope: ::prost::alloc::string::String,
    }
}
/// Deprecated. See CheckResourcesResponse.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResourceSetResponse {
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="2")]
    pub resource_instances: ::std::collections::HashMap<::prost::alloc::string::String, check_resource_set_response::ActionEffectMap>,
    #[prost(message, optional, tag="3")]
    pub meta: ::core::option::Option<check_resource_set_response::Meta>,
}
/// Nested message and enum types in `CheckResourceSetResponse`.
pub mod check_resource_set_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionEffectMap {
        #[prost(map="string, enumeration(super::super::super::effect::v1::Effect)", tag="1")]
        pub actions: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
        #[prost(message, repeated, tag="2")]
        pub validation_errors: ::prost::alloc::vec::Vec<super::super::super::schema::v1::ValidationError>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Meta {
        #[prost(map="string, message", tag="1")]
        pub resource_instances: ::std::collections::HashMap<::prost::alloc::string::String, meta::ActionMeta>,
    }
    /// Nested message and enum types in `Meta`.
    pub mod meta {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EffectMeta {
            #[prost(string, tag="1")]
            pub matched_policy: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub matched_scope: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ActionMeta {
            #[prost(map="string, message", tag="1")]
            pub actions: ::std::collections::HashMap<::prost::alloc::string::String, EffectMeta>,
            #[prost(string, repeated, tag="2")]
            pub effective_derived_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
    }
}
/// Deprecated. See CheckResourcesResponse.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResourceBatchResponse {
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub results: ::prost::alloc::vec::Vec<check_resource_batch_response::ActionEffectMap>,
}
/// Nested message and enum types in `CheckResourceBatchResponse`.
pub mod check_resource_batch_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionEffectMap {
        #[prost(string, tag="1")]
        pub resource_id: ::prost::alloc::string::String,
        #[prost(map="string, enumeration(super::super::super::effect::v1::Effect)", tag="2")]
        pub actions: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
        #[prost(message, repeated, tag="3")]
        pub validation_errors: ::prost::alloc::vec::Vec<super::super::super::schema::v1::ValidationError>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResourcesResponse {
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub results: ::prost::alloc::vec::Vec<check_resources_response::ResultEntry>,
}
/// Nested message and enum types in `CheckResourcesResponse`.
pub mod check_resources_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResultEntry {
        #[prost(message, optional, tag="1")]
        pub resource: ::core::option::Option<result_entry::Resource>,
        #[prost(map="string, enumeration(super::super::super::effect::v1::Effect)", tag="2")]
        pub actions: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
        #[prost(message, repeated, tag="3")]
        pub validation_errors: ::prost::alloc::vec::Vec<super::super::super::schema::v1::ValidationError>,
        #[prost(message, optional, tag="4")]
        pub meta: ::core::option::Option<result_entry::Meta>,
    }
    /// Nested message and enum types in `ResultEntry`.
    pub mod result_entry {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Resource {
            #[prost(string, tag="1")]
            pub id: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub kind: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub policy_version: ::prost::alloc::string::String,
            #[prost(string, tag="4")]
            pub scope: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Meta {
            #[prost(map="string, message", tag="1")]
            pub actions: ::std::collections::HashMap<::prost::alloc::string::String, meta::EffectMeta>,
            #[prost(string, repeated, tag="2")]
            pub effective_derived_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// Nested message and enum types in `Meta`.
        pub mod meta {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct EffectMeta {
                #[prost(string, tag="1")]
                pub matched_policy: ::prost::alloc::string::String,
                #[prost(string, tag="2")]
                pub matched_scope: ::prost::alloc::string::String,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundFailure {
    #[prost(message, repeated, tag="1")]
    pub errors: ::prost::alloc::vec::Vec<playground_failure::Error>,
}
/// Nested message and enum types in `PlaygroundFailure`.
pub mod playground_failure {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        #[prost(string, tag="1")]
        pub file: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub error: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundValidateResponse {
    #[prost(string, tag="1")]
    pub playground_id: ::prost::alloc::string::String,
    #[prost(oneof="playground_validate_response::Outcome", tags="2, 3")]
    pub outcome: ::core::option::Option<playground_validate_response::Outcome>,
}
/// Nested message and enum types in `PlaygroundValidateResponse`.
pub mod playground_validate_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Outcome {
        #[prost(message, tag="2")]
        Failure(super::PlaygroundFailure),
        #[prost(message, tag="3")]
        Success(()),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundTestResponse {
    #[prost(string, tag="1")]
    pub playground_id: ::prost::alloc::string::String,
    #[prost(oneof="playground_test_response::Outcome", tags="2, 3")]
    pub outcome: ::core::option::Option<playground_test_response::Outcome>,
}
/// Nested message and enum types in `PlaygroundTestResponse`.
pub mod playground_test_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Outcome {
        #[prost(message, tag="2")]
        Failure(super::PlaygroundFailure),
        #[prost(message, tag="3")]
        Results(super::super::super::policy::v1::TestResults),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundEvaluateResponse {
    #[prost(string, tag="1")]
    pub playground_id: ::prost::alloc::string::String,
    #[prost(oneof="playground_evaluate_response::Outcome", tags="2, 3")]
    pub outcome: ::core::option::Option<playground_evaluate_response::Outcome>,
}
/// Nested message and enum types in `PlaygroundEvaluateResponse`.
pub mod playground_evaluate_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EvalResult {
        #[prost(string, tag="1")]
        pub action: ::prost::alloc::string::String,
        #[prost(enumeration="super::super::super::effect::v1::Effect", tag="2")]
        pub effect: i32,
        #[prost(string, tag="3")]
        pub policy: ::prost::alloc::string::String,
        #[prost(string, repeated, tag="4")]
        pub effective_derived_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag="5")]
        pub validation_errors: ::prost::alloc::vec::Vec<super::super::super::schema::v1::ValidationError>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EvalResultList {
        #[prost(message, repeated, tag="1")]
        pub results: ::prost::alloc::vec::Vec<EvalResult>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Outcome {
        #[prost(message, tag="2")]
        Failure(super::PlaygroundFailure),
        #[prost(message, tag="3")]
        Success(EvalResultList),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaygroundProxyResponse {
    #[prost(string, tag="1")]
    pub playground_id: ::prost::alloc::string::String,
    #[prost(oneof="playground_proxy_response::Outcome", tags="2, 3, 4, 5")]
    pub outcome: ::core::option::Option<playground_proxy_response::Outcome>,
}
/// Nested message and enum types in `PlaygroundProxyResponse`.
pub mod playground_proxy_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Outcome {
        #[prost(message, tag="2")]
        Failure(super::PlaygroundFailure),
        #[prost(message, tag="3")]
        CheckResourceSet(super::CheckResourceSetResponse),
        #[prost(message, tag="4")]
        CheckResourceBatch(super::CheckResourceBatchResponse),
        #[prost(message, tag="5")]
        ResourcesQueryPlan(super::ResourcesQueryPlanResponse),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrUpdatePolicyResponse {
    #[prost(message, optional, tag="1")]
    pub success: ::core::option::Option<()>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuditLogEntriesResponse {
    #[prost(oneof="list_audit_log_entries_response::Entry", tags="1, 2")]
    pub entry: ::core::option::Option<list_audit_log_entries_response::Entry>,
}
/// Nested message and enum types in `ListAuditLogEntriesResponse`.
pub mod list_audit_log_entries_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        #[prost(message, tag="1")]
        AccessLogEntry(super::super::super::audit::v1::AccessLogEntry),
        #[prost(message, tag="2")]
        DecisionLogEntry(super::super::super::audit::v1::DecisionLogEntry),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInfoResponse {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub commit: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub build_date: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesResponse {
    #[prost(string, repeated, tag="1")]
    pub policy_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyResponse {
    #[prost(message, repeated, tag="1")]
    pub policies: ::prost::alloc::vec::Vec<super::super::policy::v1::Policy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrUpdateSchemaResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchemasResponse {
    #[prost(string, repeated, tag="1")]
    pub schema_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaResponse {
    #[prost(message, repeated, tag="1")]
    pub schemas: ::prost::alloc::vec::Vec<super::super::schema::v1::Schema>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSchemaResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadStoreResponse {
}
