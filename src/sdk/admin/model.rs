// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::{
    fs::File,
    io::{BufReader, Read},
    time::SystemTime,
};

use crate::{
    genpb::cerbos::{
        policy::{self, v1::Policy},
        response::v1::ListAuditLogEntriesResponse,
        schema::v1::Schema,
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

/// Audit log types
#[derive(Debug, Clone, Copy)]
pub enum AuditLogType {
    Access,
    Decision,
}

/// Options for audit log retrieval
#[derive(Debug, Clone)]
pub struct AuditLogOptions {
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
    pub lookup: Option<String>,
    pub tail: Option<u32>,
    pub log_type: AuditLogType,
}

/// Audit log entry wrapper
#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    response: ListAuditLogEntriesResponse,
    error: Option<String>,
}
impl PolicySet {
    /// Create a new empty policy set
    pub fn new() -> Self {
        Self::default()
    }

    /// Add policies to the set
    pub fn add_policies(&mut self, policies: Vec<Policy>) -> &mut Self {
        self.policies.extend(policies);
        self
    }

    /// Add a single policy to the set
    pub fn add_policy(&mut self, policy: Policy) -> &mut Self {
        self.policies.push(policy);
        self
    }

    /// Get all policies in the set
    pub fn get_policies(&self) -> &[Policy] {
        &self.policies
    }

    /// Get the size of the policy set
    pub fn size(&self) -> usize {
        self.policies.len()
    }

    /// Validate the policy set
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
    /// Create a new empty schema set
    pub fn new() -> Self {
        Self::default()
    }

    /// Add schemas to the set
    pub fn add_schemas(&mut self, schemas: Vec<Schema>) -> &mut Self {
        self.schemas.extend(schemas);
        self
    }

    /// Add a single schema to the set
    pub fn add_schema(&mut self, schema: Schema) -> &mut Self {
        self.schemas.push(schema);
        self
    }

    /// Get all schemas in the set
    pub fn get_schemas(&self) -> &[Schema] {
        &self.schemas
    }

    /// Get the size of the schema set
    pub fn size(&self) -> usize {
        self.schemas.len()
    }
}

impl FilterOptions {
    /// Create new filter options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set policy IDs filter
    pub fn with_policy_ids(mut self, ids: Vec<String>) -> Self {
        self.policy_ids = ids;
        self
    }

    /// Include disabled policies/schemas
    pub fn with_include_disabled(mut self, include: bool) -> Self {
        self.include_disabled = include;
        self
    }

    /// Set name regexp filter
    pub fn with_name_regexp(mut self, regexp: impl Into<String>) -> Self {
        self.name_regexp = Some(regexp.into());
        self
    }

    /// Set scope regexp filter
    pub fn with_scope_regexp(mut self, regexp: impl Into<String>) -> Self {
        self.scope_regexp = Some(regexp.into());
        self
    }

    /// Set version regexp filter
    pub fn with_version_regexp(mut self, regexp: impl Into<String>) -> Self {
        self.version_regexp = Some(regexp.into());
        self
    }
}

impl AuditLogOptions {
    /// Create new audit log options
    pub fn new(log_type: AuditLogType) -> Self {
        Self {
            start_time: None,
            end_time: None,
            lookup: None,
            tail: None,
            log_type,
        }
    }

    /// Set time range for audit logs
    pub fn with_time_range(mut self, start: SystemTime, end: SystemTime) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    /// Set lookup string for audit logs
    pub fn with_lookup(mut self, lookup: String) -> Self {
        self.lookup = Some(lookup);
        self
    }

    /// Set tail number for audit logs
    pub fn with_tail(mut self, tail: u32) -> Self {
        self.tail = Some(tail);
        self
    }
}

impl AuditLogEntry {
    /// Create a new audit log entry
    pub fn new(response: ListAuditLogEntriesResponse, error: Option<String>) -> Self {
        Self { response, error }
    }

    /// Check if there's an error
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    /// Get error message if present
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
