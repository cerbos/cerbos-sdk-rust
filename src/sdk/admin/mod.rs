// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::genpb::cerbos::svc::v1::cerbos_admin_service_client::CerbosAdminServiceClient;
use crate::genpb::cerbos::{
    policy::v1::Policy,
    request::v1::{
        AddOrUpdatePolicyRequest, AddOrUpdateSchemaRequest, DeleteSchemaRequest,
        DisablePolicyRequest, EnablePolicyRequest, GetPolicyRequest, GetSchemaRequest,
        InspectPoliciesRequest, ListAuditLogEntriesRequest, ListPoliciesRequest,
        ListSchemasRequest, ReloadStoreRequest,
    },
    response::v1::{InspectPoliciesResponse, ListAuditLogEntriesResponse},
    schema::v1::Schema,
};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::time::SystemTime;
use tonic::metadata::{MetadataKey, MetadataValue};
use tonic::service::interceptor::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};

use super::CerbosClientOptions;

const ADD_POLICY_BATCH_SIZE: usize = 10;
const ADD_SCHEMA_BATCH_SIZE: usize = 10;

/// Admin client for the Cerbos Admin API
pub struct CerbosAdminClient {
    client: CerbosAdminServiceClient<InterceptedService<Channel, BasicAuthInterceptor>>,
    headers: HashMap<String, String>,
}

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

/// Basic authentication credentials
#[derive(Debug, Clone)]
pub struct BasicAuth {
    username: String,
    password: String,
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
    pub fn with_name_regexp(mut self, regexp: String) -> Self {
        self.name_regexp = Some(regexp);
        self
    }

    /// Set scope regexp filter
    pub fn with_scope_regexp(mut self, regexp: String) -> Self {
        self.scope_regexp = Some(regexp);
        self
    }

    /// Set version regexp filter
    pub fn with_version_regexp(mut self, regexp: String) -> Self {
        self.version_regexp = Some(regexp);
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

impl BasicAuth {
    /// Create new basic auth credentials
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

/// Basic auth interceptor for gRPC requests
#[derive(Clone)]
struct BasicAuthInterceptor {
    auth_header: Option<MetadataValue<tonic::metadata::Ascii>>,
    headers: HashMap<String, String>,
}

impl BasicAuthInterceptor {
    fn new(credentials: Option<&BasicAuth>, headers: HashMap<String, String>) -> Result<Self> {
        let auth_header = if let Some(creds) = credentials {
            let auth_string = format!("{}:{}", creds.username, creds.password);
            let encoded = base64::encode(auth_string);
            let header_value = format!("Basic {}", encoded);
            Some(MetadataValue::try_from(header_value)?)
        } else {
            None
        };

        Ok(Self {
            auth_header,
            headers,
        })
    }
}

impl Interceptor for BasicAuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> std::result::Result<Request<()>, Status> {
        let metadata = request.metadata_mut();

        // Add basic auth header if present
        if let Some(ref auth_header) = self.auth_header {
            metadata.insert("authorization", auth_header.clone());
        }

        // Add custom headers
        for (key, value) in &self.headers {
            if let (Ok(key), Ok(value)) = (
                MetadataKey::from_bytes(key.as_bytes()),
                MetadataValue::try_from(value.as_str()),
            ) {
                metadata.insert(key, value);
            }
        }

        Ok(request)
    }
}

impl CerbosAdminClient {
    /// Create a new admin client with explicit credentials
    pub async fn new_with_credentials<S: Into<String> + Send>(
        endpoint: String,
        username: String,
        password: String,
        options: CerbosClientOptions<S>,
    ) -> Result<Self> {
        let credentials = Some(BasicAuth::new(username, password));

        let mut endpoint_builder = Endpoint::from_shared(endpoint.clone())
            .with_context(|| format!("Failed to create endpoint for {}", endpoint))?;

        if let Some(tls_config) = options.tls_config {
            endpoint_builder = endpoint_builder
                .tls_config(tls_config)
                .with_context(|| "Failed to apply TLS configuration")?;
        }

        endpoint_builder = endpoint_builder.timeout(options.timeout);

        endpoint_builder = endpoint_builder
            .user_agent(options.user_agent)
            .with_context(|| "Failed to set user agent")?;

        // Connect
        let channel = endpoint_builder
            .connect()
            .await
            .with_context(|| format!("Failed to connect to {}", endpoint))?;

        // Create interceptor
        let interceptor = BasicAuthInterceptor::new(credentials.as_ref(), options.headers.clone())?;

        // Create client with interceptor
        let client = CerbosAdminServiceClient::with_interceptor(channel, interceptor);

        Ok(Self {
            client,
            headers: options.headers,
        })
    }

    /// Add custom headers to the client
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Add or update policies
    pub async fn add_or_update_policy(&mut self, policies: &PolicySet) -> Result<()> {
        policies.validate()?;

        let all_policies = policies.get_policies();

        // Process policies in batches
        for chunk in all_policies.chunks(ADD_POLICY_BATCH_SIZE) {
            let request = AddOrUpdatePolicyRequest {
                policies: chunk.to_vec(),
            };

            self.client
                .add_or_update_policy(request)
                .await
                .with_context(|| "Failed to add or update policies")?;
        }

        Ok(())
    }

    /// List policies with optional filtering
    pub async fn list_policies(&mut self, options: Option<FilterOptions>) -> Result<Vec<String>> {
        let options = options.unwrap_or_default();

        let request = ListPoliciesRequest {
            policy_id: options.policy_ids,
            include_disabled: options.include_disabled,
            name_regexp: options.name_regexp.unwrap_or_default(),
            scope_regexp: options.scope_regexp.unwrap_or_default(),
            version_regexp: options.version_regexp.unwrap_or_default(),
        };

        let response = self
            .client
            .list_policies(request)
            .await
            .with_context(|| "Failed to list policies")?;

        Ok(response.into_inner().policy_ids)
    }

    /// Inspect policies with optional filtering
    pub async fn inspect_policies(
        &mut self,
        options: Option<FilterOptions>,
    ) -> Result<InspectPoliciesResponse> {
        let options = options.unwrap_or_default();

        let request = InspectPoliciesRequest {
            policy_id: options.policy_ids,
            include_disabled: options.include_disabled,
            name_regexp: options.name_regexp.unwrap_or_default(),
            scope_regexp: options.scope_regexp.unwrap_or_default(),
            version_regexp: options.version_regexp.unwrap_or_default(),
        };

        let response = self
            .client
            .inspect_policies(request)
            .await
            .with_context(|| "Failed to inspect policies")?;

        Ok(response.into_inner())
    }

    /// Get policies by IDs
    pub async fn get_policy(&mut self, ids: Vec<String>) -> Result<Vec<Policy>> {
        let request = GetPolicyRequest { id: ids };

        let response = self
            .client
            .get_policy(request)
            .await
            .with_context(|| "Failed to get policies")?;

        Ok(response.into_inner().policies)
    }

    /// Disable policies by IDs
    pub async fn disable_policy(&mut self, ids: Vec<String>) -> Result<u32> {
        let request = DisablePolicyRequest { id: ids };

        let response = self
            .client
            .disable_policy(request)
            .await
            .with_context(|| "Failed to disable policies")?;

        Ok(response.into_inner().disabled_policies)
    }

    /// Enable policies by IDs
    pub async fn enable_policy(&mut self, ids: Vec<String>) -> Result<u32> {
        let request = EnablePolicyRequest { id: ids };

        let response = self
            .client
            .enable_policy(request)
            .await
            .with_context(|| "Failed to enable policies")?;

        Ok(response.into_inner().enabled_policies)
    }

    /// Add or update schemas
    pub async fn add_or_update_schema(&mut self, schemas: &SchemaSet) -> Result<()> {
        let all_schemas = schemas.get_schemas();

        // Process schemas in batches
        for chunk in all_schemas.chunks(ADD_SCHEMA_BATCH_SIZE) {
            let request = AddOrUpdateSchemaRequest {
                schemas: chunk.to_vec(),
            };

            self.client
                .add_or_update_schema(request)
                .await
                .with_context(|| "Failed to add or update schemas")?;
        }

        Ok(())
    }

    /// Delete schemas by IDs
    pub async fn delete_schema(&mut self, ids: Vec<String>) -> Result<u32> {
        let request = DeleteSchemaRequest { id: ids };

        let response = self
            .client
            .delete_schema(request)
            .await
            .with_context(|| "Failed to delete schemas")?;

        Ok(response.into_inner().deleted_schemas)
    }

    /// List all schemas
    pub async fn list_schemas(&mut self) -> Result<Vec<String>> {
        let request = ListSchemasRequest {};

        let response = self
            .client
            .list_schemas(request)
            .await
            .with_context(|| "Failed to list schemas")?;

        Ok(response.into_inner().schema_ids)
    }

    /// Get schemas by IDs
    pub async fn get_schema(&mut self, ids: Vec<String>) -> Result<Vec<Schema>> {
        let request = GetSchemaRequest { id: ids };

        let response = self
            .client
            .get_schema(request)
            .await
            .with_context(|| "Failed to get schemas")?;

        Ok(response.into_inner().schemas)
    }

    /// Reload the policy store
    pub async fn reload_store(&mut self, wait: bool) -> Result<()> {
        let request = ReloadStoreRequest { wait };

        self.client
            .reload_store(request)
            .await
            .with_context(|| "Failed to reload store")?;

        Ok(())
    }

    /// Get audit logs (streaming)
    pub async fn audit_logs(
        &mut self,
        opts: AuditLogOptions,
    ) -> Result<tonic::Streaming<ListAuditLogEntriesResponse>> {
        use crate::genpb::cerbos::request::v1::list_audit_log_entries_request::{
            Filter, Kind, TimeRange,
        };

        let kind = match opts.log_type {
            AuditLogType::Access => Kind::Access,
            AuditLogType::Decision => Kind::Decision,
        };

        let filter = if let Some(tail) = opts.tail {
            Some(Filter::Tail(tail))
        } else if let (Some(start), Some(end)) = (opts.start_time, opts.end_time) {
            Some(Filter::Between(TimeRange {
                start: Some(start.into()),
                end: Some(end.into()),
            }))
        } else if let Some(lookup) = opts.lookup {
            Some(Filter::Lookup(lookup))
        } else {
            None
        };

        let request = ListAuditLogEntriesRequest {
            kind: kind.into(),
            filter,
        };

        let response = self
            .client
            .list_audit_log_entries(request)
            .await
            .with_context(|| "Failed to get audit logs")?;

        Ok(response.into_inner())
    }
}
