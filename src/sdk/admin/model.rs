// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    genpb::{
        cerbos::{
            policy::v1::Policy,
            request::v1::{
                list_audit_log_entries_request::{Filter, Kind, TimeRange},
                ListAuditLogEntriesRequest,
            },
            response::v1::ListAuditLogEntriesResponse,
            schema::v1::Schema,
        },
        google::protobuf::Timestamp,
    },
    sdk::deser::read_policy,
};
use anyhow::{Context, Result};

/// Policy set container for managing multiple policies
#[derive(Debug, Clone, Default)]
pub struct PolicySet {
    policies: Vec<Policy>,
}

/// Schema set container for managing multiple schemas
#[derive(Debug, Clone, Default)]
pub struct SchemaSet {
    schemas: Vec<Schema>,
}

/// Options for filtering policies and schemas
#[derive(Debug, Clone, Default)]
pub struct FilterOptions {
    pub policy_ids: Vec<String>,
    pub include_disabled: bool,
    pub name_regexp: Option<String>,
    pub scope_regexp: Option<String>,
    pub version_regexp: Option<String>,
}

impl PolicySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_policies(&mut self, policies: impl IntoIterator<Item = Policy>) -> &mut Self {
        self.policies.extend(policies);
        self
    }

    pub fn add_policy(&mut self, policy: Policy) -> &mut Self {
        self.policies.push(policy);
        self
    }

    pub fn get_policies(&self) -> &[Policy] {
        &self.policies
    }

    pub fn size(&self) -> usize {
        self.policies.len()
    }

    pub fn validate(&self) -> Result<()> {
        if self.policies.is_empty() {
            anyhow::bail!("empty policy set");
        }
        Ok(())
    }

    pub fn add_policy_from_file(&mut self, policy_path: std::path::PathBuf) -> Result<()> {
        let file = File::open(&policy_path)
            .with_context(|| format!("filed to open {}", policy_path.display()))?;
        let policy = read_policy(BufReader::new(file))
            .with_context(|| format!("failed to read policy from {}", policy_path.display()))?;
        self.add_policy(policy);
        Ok(())
    }

    pub fn add_policy_from_reader(&mut self, reader: impl Read) -> Result<()> {
        let policy = read_policy(reader)?;
        self.add_policy(policy);
        Ok(())
    }
}

impl SchemaSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_schemas(&mut self, schemas: impl IntoIterator<Item = Schema>) -> &mut Self {
        self.schemas.extend(schemas);
        self
    }

    pub fn add_schema(&mut self, schema: Schema) -> &mut Self {
        self.schemas.push(schema);
        self
    }

    pub fn get_schemas(&self) -> &[Schema] {
        &self.schemas
    }

    pub fn add_schema_from_file(
        &mut self,
        schema_path: std::path::PathBuf,
        id: impl Into<String>,
    ) -> Result<()> {
        let mut file = File::open(&schema_path)
            .with_context(|| format!("filed to open {}", schema_path.display()))?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        self.add_schema(Schema {
            id: id.into(),
            definition: buf,
        });
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.schemas.len()
    }
}

impl FilterOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_policy_ids(mut self, ids: Vec<String>) -> Self {
        self.policy_ids = ids;
        self
    }

    pub fn with_include_disabled(mut self, include: bool) -> Self {
        self.include_disabled = include;
        self
    }

    pub fn with_name_regexp(mut self, regexp: impl Into<String>) -> Self {
        self.name_regexp = Some(regexp.into());
        self
    }

    pub fn with_scope_regexp(mut self, regexp: impl Into<String>) -> Self {
        self.scope_regexp = Some(regexp.into());
        self
    }

    pub fn with_version_regexp(mut self, regexp: impl Into<String>) -> Self {
        self.version_regexp = Some(regexp.into());
        self
    }
}

impl ListAuditLogEntriesRequest {
    pub fn new() -> Self {
        Self {
            kind: Kind::Unspecified as i32,
            filter: None,
        }
    }
    pub fn with_log_entries_kind(mut self, kind: Kind) -> Self {
        self.kind = kind as i32;
        self
    }
    pub fn with_time_range(mut self, start: Timestamp, end: Timestamp) -> Self {
        self.filter = Some(Filter::Between(TimeRange {
            start: Some(start),
            end: Some(end),
        }));
        self
    }
    pub fn with_lookup(mut self, lookup: String) -> Self {
        self.filter = Some(Filter::Lookup(lookup));
        self
    }
    pub fn with_tail(mut self, tail: u32) -> Self {
        self.filter = Some(Filter::Tail(tail));
        self
    }
}
