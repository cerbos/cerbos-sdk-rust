// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    #[prost(string, tag = "1")]
    pub api_version: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub disabled: bool,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(map = "string, string", tag = "8")]
    pub variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "9")]
    pub json_schema: ::prost::alloc::string::String,
    #[prost(oneof = "policy::PolicyType", tags = "5, 6, 7, 10")]
    pub policy_type: ::core::option::Option<policy::PolicyType>,
}
/// Nested message and enum types in `Policy`.
pub mod policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PolicyType {
        #[prost(message, tag = "5")]
        ResourcePolicy(super::ResourcePolicy),
        #[prost(message, tag = "6")]
        PrincipalPolicy(super::PrincipalPolicy),
        #[prost(message, tag = "7")]
        DerivedRoles(super::DerivedRoles),
        #[prost(message, tag = "10")]
        ExportVariables(super::ExportVariables),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceAttributes {
    #[prost(map = "string, message", tag = "1")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub source_file: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "3")]
    pub hash: ::core::option::Option<u64>,
    #[deprecated]
    #[prost(string, tag = "4")]
    pub store_identifer: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub store_identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub source_attributes: ::core::option::Option<SourceAttributes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourcePolicy {
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub import_derived_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub rules: ::prost::alloc::vec::Vec<ResourceRule>,
    #[prost(string, tag = "5")]
    pub scope: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub schemas: ::core::option::Option<Schemas>,
    #[prost(message, optional, tag = "7")]
    pub variables: ::core::option::Option<Variables>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceRule {
    #[prost(string, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub derived_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub condition: ::core::option::Option<Condition>,
    #[prost(enumeration = "super::super::effect::v1::Effect", tag = "5")]
    pub effect: i32,
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub output: ::core::option::Option<Output>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrincipalPolicy {
    #[prost(string, tag = "1")]
    pub principal: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<PrincipalRule>,
    #[prost(string, tag = "4")]
    pub scope: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub variables: ::core::option::Option<Variables>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrincipalRule {
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub actions: ::prost::alloc::vec::Vec<principal_rule::Action>,
}
/// Nested message and enum types in `PrincipalRule`.
pub mod principal_rule {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        #[prost(string, tag = "1")]
        pub action: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub condition: ::core::option::Option<super::Condition>,
        #[prost(enumeration = "super::super::super::effect::v1::Effect", tag = "3")]
        pub effect: i32,
        #[prost(string, tag = "4")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "5")]
        pub output: ::core::option::Option<super::Output>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivedRoles {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub definitions: ::prost::alloc::vec::Vec<RoleDef>,
    #[prost(message, optional, tag = "3")]
    pub variables: ::core::option::Option<Variables>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleDef {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub parent_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub condition: ::core::option::Option<Condition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportVariables {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub definitions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variables {
    #[prost(string, repeated, tag = "1")]
    pub import: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, string", tag = "2")]
    pub local: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    #[prost(oneof = "condition::Condition", tags = "1, 2")]
    pub condition: ::core::option::Option<condition::Condition>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        #[prost(message, tag = "1")]
        Match(super::Match),
        #[prost(string, tag = "2")]
        Script(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    #[prost(oneof = "r#match::Op", tags = "1, 2, 3, 4")]
    pub op: ::core::option::Option<r#match::Op>,
}
/// Nested message and enum types in `Match`.
pub mod r#match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExprList {
        #[prost(message, repeated, tag = "1")]
        pub of: ::prost::alloc::vec::Vec<super::Match>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Op {
        #[prost(message, tag = "1")]
        All(ExprList),
        #[prost(message, tag = "2")]
        Any(ExprList),
        #[prost(message, tag = "3")]
        None(ExprList),
        #[prost(string, tag = "4")]
        Expr(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub expr: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub when: ::core::option::Option<output::When>,
}
/// Nested message and enum types in `Output`.
pub mod output {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct When {
        #[prost(string, tag = "1")]
        pub rule_activated: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub condition_not_met: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schemas {
    #[prost(message, optional, tag = "1")]
    pub principal_schema: ::core::option::Option<schemas::Schema>,
    #[prost(message, optional, tag = "2")]
    pub resource_schema: ::core::option::Option<schemas::Schema>,
}
/// Nested message and enum types in `Schemas`.
pub mod schemas {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IgnoreWhen {
        #[prost(string, repeated, tag = "1")]
        pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Schema {
        #[prost(string, tag = "1")]
        pub r#ref: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub ignore_when: ::core::option::Option<IgnoreWhen>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFixture {}
/// Nested message and enum types in `TestFixture`.
pub mod test_fixture {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Principals {
        #[prost(map = "string, message", tag = "1")]
        pub principals: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::super::super::engine::v1::Principal,
        >,
        #[prost(string, tag = "2")]
        pub json_schema: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resources {
        #[prost(map = "string, message", tag = "1")]
        pub resources: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::super::super::engine::v1::Resource,
        >,
        #[prost(string, tag = "2")]
        pub json_schema: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AuxData {
        #[prost(map = "string, message", tag = "1")]
        pub aux_data: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::super::super::engine::v1::AuxData,
        >,
        #[prost(string, tag = "2")]
        pub json_schema: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestOptions {
    #[prost(message, optional, tag = "1")]
    pub now: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "2")]
    pub lenient_scope_search: bool,
    #[prost(map = "string, message", tag = "3")]
    pub globals: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSuite {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub skip: bool,
    #[prost(string, tag = "4")]
    pub skip_reason: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub tests: ::prost::alloc::vec::Vec<TestTable>,
    #[prost(map = "string, message", tag = "6")]
    pub principals: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::engine::v1::Principal,
    >,
    #[prost(map = "string, message", tag = "7")]
    pub resources: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::engine::v1::Resource,
    >,
    #[prost(map = "string, message", tag = "8")]
    pub aux_data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::engine::v1::AuxData,
    >,
    #[prost(message, optional, tag = "9")]
    pub options: ::core::option::Option<TestOptions>,
    #[prost(string, tag = "10")]
    pub json_schema: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestTable {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub skip: bool,
    #[prost(string, tag = "4")]
    pub skip_reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub input: ::core::option::Option<test_table::Input>,
    #[prost(message, repeated, tag = "6")]
    pub expected: ::prost::alloc::vec::Vec<test_table::Expectation>,
    #[prost(message, optional, tag = "7")]
    pub options: ::core::option::Option<TestOptions>,
}
/// Nested message and enum types in `TestTable`.
pub mod test_table {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(string, repeated, tag = "1")]
        pub principals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "2")]
        pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "3")]
        pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "4")]
        pub aux_data: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputExpectations {
        #[prost(string, tag = "1")]
        pub action: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub expected: ::prost::alloc::vec::Vec<
            super::super::super::engine::v1::OutputEntry,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Expectation {
        #[prost(string, tag = "1")]
        pub principal: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub resource: ::prost::alloc::string::String,
        #[prost(
            map = "string, enumeration(super::super::super::effect::v1::Effect)",
            tag = "3"
        )]
        pub actions: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
        #[prost(message, repeated, tag = "4")]
        pub outputs: ::prost::alloc::vec::Vec<OutputExpectations>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Test {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<test::TestName>,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub skip: bool,
    #[prost(string, tag = "4")]
    pub skip_reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub input: ::core::option::Option<super::super::engine::v1::CheckInput>,
    #[prost(map = "string, enumeration(super::super::effect::v1::Effect)", tag = "6")]
    pub expected: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    #[prost(message, optional, tag = "7")]
    pub options: ::core::option::Option<TestOptions>,
    #[prost(map = "string, message", tag = "8")]
    pub expected_outputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        test::OutputEntries,
    >,
}
/// Nested message and enum types in `Test`.
pub mod test {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TestName {
        #[prost(string, tag = "1")]
        pub test_table_name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub principal_key: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub resource_key: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputEntries {
        #[prost(map = "string, message", tag = "1")]
        pub entries: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost_types::Value,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestResults {
    #[prost(message, repeated, tag = "1")]
    pub suites: ::prost::alloc::vec::Vec<test_results::Suite>,
    #[prost(message, optional, tag = "2")]
    pub summary: ::core::option::Option<test_results::Summary>,
}
/// Nested message and enum types in `TestResults`.
pub mod test_results {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tally {
        #[prost(enumeration = "Result", tag = "1")]
        pub result: i32,
        #[prost(uint32, tag = "2")]
        pub count: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Summary {
        #[prost(enumeration = "Result", tag = "1")]
        pub overall_result: i32,
        #[prost(uint32, tag = "2")]
        pub tests_count: u32,
        #[prost(message, repeated, tag = "3")]
        pub result_counts: ::prost::alloc::vec::Vec<Tally>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Suite {
        #[prost(string, tag = "1")]
        pub file: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        #[deprecated]
        #[prost(message, repeated, tag = "3")]
        pub principals: ::prost::alloc::vec::Vec<Principal>,
        #[prost(message, optional, tag = "4")]
        pub summary: ::core::option::Option<Summary>,
        #[prost(string, tag = "5")]
        pub error: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "6")]
        pub test_cases: ::prost::alloc::vec::Vec<TestCase>,
        #[prost(string, tag = "7")]
        pub description: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TestCase {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub principals: ::prost::alloc::vec::Vec<Principal>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Principal {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub resources: ::prost::alloc::vec::Vec<Resource>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub actions: ::prost::alloc::vec::Vec<Action>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub details: ::core::option::Option<Details>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Details {
        #[prost(enumeration = "Result", tag = "1")]
        pub result: i32,
        #[prost(message, repeated, tag = "4")]
        pub engine_trace: ::prost::alloc::vec::Vec<
            super::super::super::engine::v1::Trace,
        >,
        #[prost(oneof = "details::Outcome", tags = "2, 3, 5")]
        pub outcome: ::core::option::Option<details::Outcome>,
    }
    /// Nested message and enum types in `Details`.
    pub mod details {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Outcome {
            #[prost(message, tag = "2")]
            Failure(super::Failure),
            #[prost(string, tag = "3")]
            Error(::prost::alloc::string::String),
            #[prost(message, tag = "5")]
            Success(super::Success),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputFailure {
        #[prost(string, tag = "1")]
        pub src: ::prost::alloc::string::String,
        #[prost(oneof = "output_failure::Outcome", tags = "2, 3")]
        pub outcome: ::core::option::Option<output_failure::Outcome>,
    }
    /// Nested message and enum types in `OutputFailure`.
    pub mod output_failure {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MismatchedValue {
            #[prost(message, optional, tag = "1")]
            pub expected: ::core::option::Option<::prost_types::Value>,
            #[prost(message, optional, tag = "2")]
            pub actual: ::core::option::Option<::prost_types::Value>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MissingValue {
            #[prost(message, optional, tag = "1")]
            pub expected: ::core::option::Option<::prost_types::Value>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Outcome {
            #[prost(message, tag = "2")]
            Mismatched(MismatchedValue),
            #[prost(message, tag = "3")]
            Missing(MissingValue),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Failure {
        #[prost(enumeration = "super::super::super::effect::v1::Effect", tag = "1")]
        pub expected: i32,
        #[prost(enumeration = "super::super::super::effect::v1::Effect", tag = "2")]
        pub actual: i32,
        #[prost(message, repeated, tag = "3")]
        pub outputs: ::prost::alloc::vec::Vec<OutputFailure>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Success {
        #[prost(enumeration = "super::super::super::effect::v1::Effect", tag = "1")]
        pub effect: i32,
        #[prost(message, repeated, tag = "2")]
        pub outputs: ::prost::alloc::vec::Vec<
            super::super::super::engine::v1::OutputEntry,
        >,
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
    pub enum Result {
        Unspecified = 0,
        Skipped = 1,
        Passed = 2,
        Failed = 3,
        Errored = 4,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unspecified => "RESULT_UNSPECIFIED",
                Result::Skipped => "RESULT_SKIPPED",
                Result::Passed => "RESULT_PASSED",
                Result::Failed => "RESULT_FAILED",
                Result::Errored => "RESULT_ERRORED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESULT_UNSPECIFIED" => Some(Self::Unspecified),
                "RESULT_SKIPPED" => Some(Self::Skipped),
                "RESULT_PASSED" => Some(Self::Passed),
                "RESULT_FAILED" => Some(Self::Failed),
                "RESULT_ERRORED" => Some(Self::Errored),
                _ => None,
            }
        }
    }
}
