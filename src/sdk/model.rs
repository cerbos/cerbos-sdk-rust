// Copyright 2021-2022 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
use super::attr::{AttrVal, Attribute};
use crate::genpb::cerbos::effect::v1::Effect;
use crate::genpb::cerbos::engine::v1::{
    plan_resources_request::Resource as ResourceKindPB, Principal as PrincipalPB,
    Resource as ResourcePB,
};
use crate::genpb::cerbos::request::v1::aux_data::Jwt;
use crate::genpb::cerbos::request::v1::check_resources_request::ResourceEntry;
use crate::genpb::cerbos::request::v1::AuxData as AuxDataPB;
use crate::genpb::cerbos::response::v1::check_resources_response::ResultEntry;
use crate::genpb::cerbos::response::v1::plan_resources_response::{
    expression::Operand, filter::Kind,
};
use crate::genpb::cerbos::response::v1::{
    CheckResourcesResponse as CheckResourcesResponsePB,
    PlanResourcesResponse as PlanResourcesResponsePB,
};
use prost::Message;
use std::slice::Iter;

pub(crate) trait ProtobufWrapper<T: Message> {
    fn to_pb(self) -> T;
}

#[derive(Debug, Clone)]
pub struct Principal {
    pub(crate) principal: PrincipalPB,
}

impl Principal {
    pub fn new<I, T>(id: T, roles: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let principal = PrincipalPB {
            id: id.into(),
            roles: roles.into_iter().map(Into::into).collect::<Vec<String>>(),
            ..Default::default()
        };

        Principal { principal }
    }

    pub fn id(&self) -> &str {
        &self.principal.id
    }

    pub fn add_role(mut self, role: impl Into<String>) -> Self {
        self.principal.roles.push(role.into());
        self
    }

    pub fn with_policy_version(mut self, policy_version: impl Into<String>) -> Self {
        self.principal.policy_version = policy_version.into();
        self
    }

    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.principal.scope = scope.into();
        self
    }

    pub fn with_attributes<I>(mut self, attrs: I) -> Self
    where
        I: IntoIterator<Item = Attribute>,
    {
        self.principal
            .attr
            .extend(attrs.into_iter().map(Attribute::into_tuple));
        self
    }

    pub fn add_attr(mut self, key: impl Into<String>, value: impl AttrVal) -> Self {
        self.principal.attr.insert(key.into(), value.to_value());
        self
    }
}

impl ProtobufWrapper<PrincipalPB> for Principal {
    fn to_pb(self) -> PrincipalPB {
        self.principal
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub(crate) resource: ResourcePB,
}

impl Resource {
    pub fn new(id: impl Into<String>, kind: impl Into<String>) -> Self {
        let resource = ResourcePB {
            id: id.into(),
            kind: kind.into(),
            ..Default::default()
        };

        Resource { resource }
    }

    pub fn with_policy_version(mut self, policy_version: impl Into<String>) -> Self {
        self.resource.policy_version = policy_version.into();
        self
    }

    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.resource.scope = scope.into();
        self
    }

    pub fn with_attributes<I>(mut self, attrs: I) -> Self
    where
        I: IntoIterator<Item = Attribute>,
    {
        self.resource
            .attr
            .extend(attrs.into_iter().map(Attribute::into_tuple));
        self
    }

    pub fn add_attr(mut self, key: impl Into<String>, value: impl AttrVal) -> Self {
        self.resource.attr.insert(key.into(), value.to_value());
        self
    }

    pub fn id(&self) -> &str {
        &self.resource.id
    }
}

impl ProtobufWrapper<ResourcePB> for Resource {
    fn to_pb(self) -> ResourcePB {
        self.resource
    }
}

#[derive(Debug, Clone)]
pub struct ResourceKind {
    pub(crate) resource: ResourceKindPB,
}

impl ResourceKind {
    pub fn new(kind: impl Into<String>) -> Self {
        let resource = ResourceKindPB {
            kind: kind.into(),
            ..Default::default()
        };

        ResourceKind { resource }
    }

    pub fn with_policy_version(mut self, policy_version: impl Into<String>) -> Self {
        self.resource.policy_version = policy_version.into();
        self
    }

    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.resource.scope = scope.into();
        self
    }

    pub fn with_attributes<I>(mut self, attrs: I) -> Self
    where
        I: IntoIterator<Item = Attribute>,
    {
        self.resource
            .attr
            .extend(attrs.into_iter().map(Attribute::into_tuple));
        self
    }

    pub fn add_attr(mut self, key: impl Into<String>, value: impl AttrVal) -> Self {
        self.resource.attr.insert(key.into(), value.to_value());
        self
    }
}

impl ProtobufWrapper<ResourceKindPB> for ResourceKind {
    fn to_pb(self) -> ResourceKindPB {
        self.resource
    }
}

impl From<Resource> for ResourceKind {
    fn from(r: Resource) -> Self {
        let resource = ResourceKindPB {
            kind: r.resource.kind,
            policy_version: r.resource.policy_version,
            scope: r.resource.scope,
            attr: r.resource.attr,
        };

        Self { resource }
    }
}

#[derive(Debug, Clone)]
pub struct AuxData {
    pub(crate) aux_data: AuxDataPB,
}

impl Default for AuxData {
    fn default() -> Self {
        Self::new()
    }
}

impl AuxData {
    pub fn new() -> AuxData {
        AuxData {
            aux_data: AuxDataPB::default(),
        }
    }

    pub fn with_jwt<T: Into<String>>(mut self, token: T, key_set_id: Option<T>) -> Self {
        let mut jwt = Jwt {
            token: token.into(),
            ..Default::default()
        };

        key_set_id
            .map(Into::into)
            .iter()
            .for_each(|ks| jwt.key_set_id = ks.to_string());

        self.aux_data.jwt = Some(jwt);
        self
    }
}

impl ProtobufWrapper<AuxDataPB> for AuxData {
    fn to_pb(self) -> AuxDataPB {
        self.aux_data
    }
}

#[derive(Debug, Clone)]
pub struct ResourceAction<A, B>(pub Resource, pub B)
where
    A: Into<String>,
    B: IntoIterator<Item = A>;

impl<A, B> ProtobufWrapper<ResourceEntry> for ResourceAction<A, B>
where
    A: Into<String>,
    B: IntoIterator<Item = A>,
{
    fn to_pb(self) -> ResourceEntry {
        ResourceEntry {
            resource: Some(self.0.to_pb()),
            actions: self.1.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResourceList {
    pub(crate) resources: Vec<ResourceEntry>,
}

impl Default for ResourceList {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceList {
    pub fn new() -> Self {
        ResourceList {
            resources: Vec::new(),
        }
    }

    pub fn new_from<A, B, I>(items: I) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = A>,
        I: IntoIterator<Item = ResourceAction<A, B>>,
    {
        Self {
            resources: items.into_iter().map(|item| item.to_pb()).collect(),
        }
    }

    pub fn add<I, T>(mut self, resource: Resource, actions: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let r = ResourceEntry {
            resource: Some(resource.to_pb()),
            actions: actions.into_iter().map(Into::into).collect(),
        };

        self.resources.push(r);
        self
    }
}

#[derive(Debug)]
pub enum ResourceMatcher {
    Kind(String),
    PolicyVersion(String),
    Scope(String),
}

#[derive(Debug)]
pub struct ResourceResult<'a> {
    pub(crate) result: &'a ResultEntry,
}

impl<'a> ResourceResult<'a> {
    pub fn is_allowed(&self, action: impl AsRef<str>) -> bool {
        self.result
            .actions
            .get(action.as_ref())
            .map_or(false, |effect| {
                Effect::Allow == Effect::from_i32(*effect).unwrap()
            })
    }
}

#[derive(Debug, Clone)]
pub struct CheckResourcesResponse {
    pub response: CheckResourcesResponsePB,
}

impl CheckResourcesResponse {
    pub fn find(&self, id: impl AsRef<str>) -> Option<ResourceResult> {
        let id_str = id.as_ref();
        let entry = self
            .response
            .results
            .iter()
            .find(|r| r.resource.as_ref().map_or(false, |rr| rr.id == id_str));

        entry.map(|r| ResourceResult { result: r })
    }

    pub fn find_with_predicates(
        &self,
        id: impl AsRef<str>,
        predicates: Vec<ResourceMatcher>,
    ) -> Option<ResourceResult> {
        let id_str = id.as_ref();
        let entry = self.response.results.iter().find(|r| {
            r.resource.as_ref().map_or(false, |rr| {
                if rr.id != id_str {
                    return false;
                }

                predicates.iter().fold(false, |acc, p| {
                    if !acc {
                        return false;
                    }

                    match p {
                        ResourceMatcher::Kind(kind) => acc && (&rr.kind == kind),
                        ResourceMatcher::PolicyVersion(version) => {
                            acc && (&rr.policy_version == version)
                        }
                        ResourceMatcher::Scope(scope) => acc && (&rr.scope == scope),
                    }
                })
            })
        });

        entry.map(|r| ResourceResult { result: r })
    }

    pub fn iter(&self) -> CheckResourcesResponseIter {
        CheckResourcesResponseIter {
            iter: self.response.results.iter(),
        }
    }
}

pub struct CheckResourcesResponseIter<'a> {
    iter: Iter<'a, ResultEntry>,
}

impl<'a> Iterator for CheckResourcesResponseIter<'a> {
    type Item = ResourceResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|r| ResourceResult { result: r })
    }
}

#[derive(Debug, Clone)]
pub struct PlanResourcesResponse {
    pub response: PlanResourcesResponsePB,
}

impl PlanResourcesResponse {
    pub fn filter(&self) -> PlanResourcesFilter {
        let f = self.response.filter.as_ref().unwrap();
        let kind = Kind::from_i32(f.kind).unwrap();

        match kind {
            Kind::AlwaysAllowed => PlanResourcesFilter::AlwaysAllowed,
            Kind::AlwaysDenied => PlanResourcesFilter::AlwaysDenied,
            Kind::Conditional => {
                PlanResourcesFilter::Conditional(f.condition.as_ref().unwrap().clone())
            }
            _ => PlanResourcesFilter::AlwaysDenied, // can never happen
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlanResourcesFilter {
    AlwaysAllowed,
    AlwaysDenied,
    Conditional(Operand),
}
