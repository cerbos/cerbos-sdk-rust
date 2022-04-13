use std::collections::HashMap;
use std::slice::Iter;

use prost::Message;
use prost_types::{value::Kind, Value};

use crate::genpb::cerbos::effect::v1::Effect;
use crate::genpb::cerbos::engine::v1::{Principal as PrincipalPB, Resource as ResourcePB};
use crate::genpb::cerbos::request::v1::aux_data::Jwt;
use crate::genpb::cerbos::request::v1::check_resources_request::ResourceEntry;
use crate::genpb::cerbos::request::v1::AuxData as AuxDataPB;
use crate::genpb::cerbos::response::v1::check_resources_response::ResultEntry;
use crate::genpb::cerbos::response::v1::CheckResourcesResponse as CheckResourcesResponsePB;

pub(crate) trait ProtobufWrapper<T: Message> {
    fn to_pb(self) -> T;
}

#[derive(Debug)]
pub struct Principal {
    pub(crate) principal: PrincipalPB,
}

impl Principal {
    pub fn new<I, T>(id: T, roles: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let mut p = PrincipalPB::default();
        p.id = id.into();
        p.roles = roles.into_iter().map(Into::into).collect::<Vec<String>>();

        Principal { principal: p }
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

    pub fn with_attributes(mut self, attrs: HashMap<String, Kind>) -> Self {
        self.principal.attr.extend(attrs.iter().map(|(k, v)| {
            (
                k.to_owned(),
                Value {
                    kind: Some(v.to_owned()),
                },
            )
        }));
        self
    }

    pub fn add_attr(mut self, key: impl Into<String>, value: Kind) -> Self {
        self.principal
            .attr
            .insert(key.into(), Value { kind: Some(value) });
        self
    }
}

impl ProtobufWrapper<PrincipalPB> for Principal {
    fn to_pb(self) -> PrincipalPB {
        self.principal
    }
}

#[derive(Debug)]
pub struct Resource {
    pub(crate) resource: ResourcePB,
}

impl Resource {
    pub fn new(id: impl Into<String>, kind: impl Into<String>) -> Self {
        let mut r = ResourcePB::default();
        r.id = id.into();
        r.kind = kind.into();

        Resource { resource: r }
    }

    pub fn with_policy_version(mut self, policy_version: impl Into<String>) -> Self {
        self.resource.policy_version = policy_version.into();
        self
    }

    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.resource.scope = scope.into();
        self
    }

    pub fn with_attributes(mut self, attrs: HashMap<String, Kind>) -> Self {
        self.resource.attr.extend(attrs.iter().map(|(k, v)| {
            (
                k.to_owned(),
                Value {
                    kind: Some(v.to_owned()),
                },
            )
        }));
        self
    }

    pub fn add_attr(mut self, key: impl Into<String>, value: Kind) -> Self {
        self.resource
            .attr
            .insert(key.into(), Value { kind: Some(value) });
        self
    }
}

impl ProtobufWrapper<ResourcePB> for Resource {
    fn to_pb(self) -> ResourcePB {
        self.resource
    }
}

#[derive(Debug)]
pub struct AuxData {
    pub(crate) aux_data: AuxDataPB,
}

impl AuxData {
    pub fn new() -> AuxData {
        AuxData {
            aux_data: AuxDataPB::default(),
        }
    }

    pub fn with_jwt<T: Into<String>>(mut self, token: T, key_set_id: Option<T>) -> Self {
        let mut jwt = Jwt::default();
        jwt.token = token.into();
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

#[derive(Debug)]
pub struct ResourceList {
    pub(crate) resources: Vec<ResourceEntry>,
}

impl ResourceList {
    pub fn new() -> Self {
        ResourceList {
            resources: Vec::new(),
        }
    }

    pub fn add<I, T>(mut self, resource: Resource, actions: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let mut r = ResourceEntry::default();
        r.resource = Some(resource.to_pb());
        r.actions = actions.into_iter().map(Into::into).collect();
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

#[derive(Debug)]
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
