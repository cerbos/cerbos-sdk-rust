#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    #[prost(string, tag="1")]
    pub api_version: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub disabled: bool,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub metadata: ::core::option::Option<Metadata>,
    #[prost(map="string, string", tag="8")]
    pub variables: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(oneof="policy::PolicyType", tags="5, 6, 7")]
    pub policy_type: ::core::option::Option<policy::PolicyType>,
}
/// Nested message and enum types in `Policy`.
pub mod policy {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PolicyType {
        #[prost(message, tag="5")]
        ResourcePolicy(super::ResourcePolicy),
        #[prost(message, tag="6")]
        PrincipalPolicy(super::PrincipalPolicy),
        #[prost(message, tag="7")]
        DerivedRoles(super::DerivedRoles),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag="1")]
    pub source_file: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="2")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub hash: ::core::option::Option<u64>,
    #[prost(string, tag="4")]
    pub store_identifer: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourcePolicy {
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub import_derived_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub rules: ::prost::alloc::vec::Vec<ResourceRule>,
    #[prost(string, tag="5")]
    pub scope: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub schemas: ::core::option::Option<Schemas>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceRule {
    #[prost(string, repeated, tag="1")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub derived_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub condition: ::core::option::Option<Condition>,
    #[prost(enumeration="super::super::effect::v1::Effect", tag="5")]
    pub effect: i32,
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrincipalPolicy {
    #[prost(string, tag="1")]
    pub principal: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub rules: ::prost::alloc::vec::Vec<PrincipalRule>,
    #[prost(string, tag="4")]
    pub scope: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrincipalRule {
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub actions: ::prost::alloc::vec::Vec<principal_rule::Action>,
}
/// Nested message and enum types in `PrincipalRule`.
pub mod principal_rule {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        #[prost(string, tag="1")]
        pub action: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub condition: ::core::option::Option<super::Condition>,
        #[prost(enumeration="super::super::super::effect::v1::Effect", tag="3")]
        pub effect: i32,
        #[prost(string, tag="4")]
        pub name: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivedRoles {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub definitions: ::prost::alloc::vec::Vec<RoleDef>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleDef {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub parent_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub condition: ::core::option::Option<Condition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    #[prost(oneof="condition::Condition", tags="1, 2")]
    pub condition: ::core::option::Option<condition::Condition>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        #[prost(message, tag="1")]
        Match(super::Match),
        #[prost(string, tag="2")]
        Script(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    #[prost(oneof="r#match::Op", tags="1, 2, 3, 4")]
    pub op: ::core::option::Option<r#match::Op>,
}
/// Nested message and enum types in `Match`.
pub mod r#match {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExprList {
        #[prost(message, repeated, tag="1")]
        pub of: ::prost::alloc::vec::Vec<super::Match>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Op {
        #[prost(message, tag="1")]
        All(ExprList),
        #[prost(message, tag="2")]
        Any(ExprList),
        #[prost(message, tag="3")]
        None(ExprList),
        #[prost(string, tag="4")]
        Expr(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schemas {
    #[prost(message, optional, tag="1")]
    pub principal_schema: ::core::option::Option<schemas::Schema>,
    #[prost(message, optional, tag="2")]
    pub resource_schema: ::core::option::Option<schemas::Schema>,
}
/// Nested message and enum types in `Schemas`.
pub mod schemas {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IgnoreWhen {
        #[prost(string, repeated, tag="1")]
        pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Schema {
        #[prost(string, tag="1")]
        pub r#ref: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub ignore_when: ::core::option::Option<IgnoreWhen>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFixture {
}
/// Nested message and enum types in `TestFixture`.
pub mod test_fixture {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Principals {
        #[prost(map="string, message", tag="1")]
        pub principals: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::engine::v1::Principal>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resources {
        #[prost(map="string, message", tag="1")]
        pub resources: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::engine::v1::Resource>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AuxData {
        #[prost(map="string, message", tag="1")]
        pub aux_data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::engine::v1::AuxData>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSuite {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub skip: bool,
    #[prost(string, tag="4")]
    pub skip_reason: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub tests: ::prost::alloc::vec::Vec<TestTable>,
    #[prost(map="string, message", tag="6")]
    pub principals: ::std::collections::HashMap<::prost::alloc::string::String, super::super::engine::v1::Principal>,
    #[prost(map="string, message", tag="7")]
    pub resources: ::std::collections::HashMap<::prost::alloc::string::String, super::super::engine::v1::Resource>,
    #[prost(map="string, message", tag="8")]
    pub aux_data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::engine::v1::AuxData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestTable {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub skip: bool,
    #[prost(string, tag="4")]
    pub skip_reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub input: ::core::option::Option<test_table::Input>,
    #[prost(message, repeated, tag="6")]
    pub expected: ::prost::alloc::vec::Vec<test_table::Expectation>,
}
/// Nested message and enum types in `TestTable`.
pub mod test_table {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(string, repeated, tag="1")]
        pub principals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag="2")]
        pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag="3")]
        pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag="4")]
        pub aux_data: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Expectation {
        #[prost(string, tag="1")]
        pub principal: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub resource: ::prost::alloc::string::String,
        #[prost(map="string, enumeration(super::super::super::effect::v1::Effect)", tag="3")]
        pub actions: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Test {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<test::TestName>,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub skip: bool,
    #[prost(string, tag="4")]
    pub skip_reason: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub input: ::core::option::Option<super::super::engine::v1::CheckInput>,
    #[prost(map="string, enumeration(super::super::effect::v1::Effect)", tag="6")]
    pub expected: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
/// Nested message and enum types in `Test`.
pub mod test {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TestName {
        #[prost(string, tag="1")]
        pub test_table_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub principal_key: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub resource_key: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestResults {
    #[prost(message, repeated, tag="1")]
    pub suites: ::prost::alloc::vec::Vec<test_results::Suite>,
    #[prost(message, optional, tag="2")]
    pub summary: ::core::option::Option<test_results::Summary>,
}
/// Nested message and enum types in `TestResults`.
pub mod test_results {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tally {
        #[prost(enumeration="Result", tag="1")]
        pub result: i32,
        #[prost(uint32, tag="2")]
        pub count: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Summary {
        #[prost(enumeration="Result", tag="1")]
        pub overall_result: i32,
        #[prost(uint32, tag="2")]
        pub tests_count: u32,
        #[prost(message, repeated, tag="3")]
        pub result_counts: ::prost::alloc::vec::Vec<Tally>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Suite {
        #[prost(string, tag="1")]
        pub file: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="3")]
        pub principals: ::prost::alloc::vec::Vec<Principal>,
        #[prost(message, optional, tag="4")]
        pub summary: ::core::option::Option<Summary>,
        #[prost(string, tag="5")]
        pub error: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Principal {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="2")]
        pub resources: ::prost::alloc::vec::Vec<Resource>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="2")]
        pub actions: ::prost::alloc::vec::Vec<Action>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag="2")]
        pub details: ::core::option::Option<Details>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Details {
        #[prost(enumeration="Result", tag="1")]
        pub result: i32,
        #[prost(message, repeated, tag="4")]
        pub engine_trace: ::prost::alloc::vec::Vec<super::super::super::engine::v1::Trace>,
        #[prost(oneof="details::Outcome", tags="2, 3")]
        pub outcome: ::core::option::Option<details::Outcome>,
    }
    /// Nested message and enum types in `Details`.
    pub mod details {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Outcome {
            #[prost(message, tag="2")]
            Failure(super::Failure),
            #[prost(string, tag="3")]
            Error(::prost::alloc::string::String),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Failure {
        #[prost(enumeration="super::super::super::effect::v1::Effect", tag="1")]
        pub expected: i32,
        #[prost(enumeration="super::super::super::effect::v1::Effect", tag="2")]
        pub actual: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        Unspecified = 0,
        Skipped = 1,
        Passed = 2,
        Failed = 3,
        Errored = 4,
    }
}
