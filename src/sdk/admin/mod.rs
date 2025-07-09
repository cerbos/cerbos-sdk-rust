// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

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
    svc::v1::cerbos_admin_service_client::CerbosAdminServiceClient,
};
use anyhow::{anyhow, bail, Context, Result};
use base64::prelude::{Engine as _, BASE64_STANDARD};
use model::{FilterOptions, PolicySet, SchemaSet};
use std::time::Duration;
use tonic::{
    metadata::MetadataValue,
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
    Request, Status,
};

use super::CerbosClientOptions;

const ADD_POLICY_BATCH_SIZE: usize = 10;
const ADD_SCHEMA_BATCH_SIZE: usize = 10;

pub mod model;

/// Basic authentication credentials
#[derive(Debug, Clone)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    /// Create new basic auth credentials
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

struct CerbosBasicAuthInterceptor {
    request_timeout: Duration,
    auth_header: MetadataValue<tonic::metadata::Ascii>,
}

impl Interceptor for CerbosBasicAuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> std::result::Result<Request<()>, Status> {
        let metadata = request.metadata_mut();
        metadata.insert("authorization", self.auth_header.clone());

        request.set_timeout(self.request_timeout);
        Ok(request)
    }
}
pub struct CerbosAdminClient {
    client: CerbosAdminServiceClient<InterceptedService<Channel, CerbosBasicAuthInterceptor>>,
}
impl CerbosAdminClient {
    /// Create a new Cerbos client using client options
    pub async fn new<S>(conf: CerbosClientOptions<S>) -> Result<Self>
    where
        S: Into<String> + Send,
    {
        let basic_auth = conf
            .admin_creds
            .as_ref()
            .ok_or_else(|| anyhow!("admin credentials required"))?;
        let auth_header = Self::make_auth_header(basic_auth)?;
        let request_timeout = conf.timeout;
        let channel = conf.build_channel()?;
        let client = CerbosAdminServiceClient::with_interceptor(
            channel,
            CerbosBasicAuthInterceptor {
                request_timeout,
                auth_header,
            },
        );

        Ok(Self { client })
    }
    fn make_auth_header(
        auth_creds: &BasicAuth,
    ) -> anyhow::Result<MetadataValue<tonic::metadata::Ascii>> {
        let auth_string = format!("{}:{}", auth_creds.username, auth_creds.password);
        let encoded = BASE64_STANDARD.encode(auth_string);
        let header_value = format!("Basic {}", encoded);
        MetadataValue::try_from(header_value).with_context(|| "fail to parse metadata value")
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
        request: ListAuditLogEntriesRequest,
    ) -> Result<tonic::Streaming<ListAuditLogEntriesResponse>> {
        use crate::genpb::cerbos::request::v1::list_audit_log_entries_request::{Filter, Kind};
        if request.kind() != Kind::Access && request.kind() != Kind::Decision {
            bail!("incorrect audit log kind");
        }
        const MAX_TAIL: u32 = 1000;
        match request.filter {
            Some(Filter::Tail(tail)) if tail > MAX_TAIL => {
                bail!("tail must not exceed {}", MAX_TAIL)
            }
            None => bail!("filter is not specified"),
            _ => {}
        };

        let response = self
            .client
            .list_audit_log_entries(request)
            .await
            .with_context(|| "Failed to get audit logs")?;

        Ok(response.into_inner())
    }
}
