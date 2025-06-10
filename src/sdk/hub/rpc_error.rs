// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
use prost::Message;
use prost_types::Any;
use std::error::Error;
use std::fmt;
use thiserror::Error;
use tonic::{metadata::MetadataMap, Code, Status};

/// Custom error type for authentication failures
#[derive(Debug, Error)]
#[error("Authentication failed")]
pub struct AuthenticationFailedError;

/// Enum representing different kinds of RPC errors that can occur
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RPCErrorKind {
    AuthenticationFailed,
    ConditionUnsatisfied,
    InvalidRequest,
    NoUsableFiles,
    OperationDiscarded,
    PermissionDenied,
    StoreNotFound,
    Unknown,
    ValidationFailure,
}

impl fmt::Display for RPCErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RPCErrorKind::AuthenticationFailed => write!(f, "Authentication failed"),
            RPCErrorKind::ConditionUnsatisfied => write!(f, "Condition unsatisfied"),
            RPCErrorKind::InvalidRequest => write!(f, "Invalid request"),
            RPCErrorKind::NoUsableFiles => write!(f, "No usable files"),
            RPCErrorKind::OperationDiscarded => write!(f, "Operation discarded"),
            RPCErrorKind::PermissionDenied => write!(f, "Permission denied"),
            RPCErrorKind::StoreNotFound => write!(f, "Store not found"),
            RPCErrorKind::Unknown => write!(f, "Unknown error"),
            RPCErrorKind::ValidationFailure => write!(f, "Validation failure"),
        }
    }
}

#[derive(Debug)]
pub struct RPCError {
    pub underlying: Box<dyn Error + Send + Sync>,
    pub ignored_files: Vec<String>,
    pub validation_errors: Vec<FileError>,
    pub kind: RPCErrorKind,
    pub current_store_version: i64,
}

impl fmt::Display for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.underlying)
    }
}

impl Error for RPCError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.underlying.as_ref())
    }
}

impl RPCError {
    /// Creates a new RPCError from any error type
    pub fn new(err: Box<dyn Error + Send + Sync>) -> Self {
        // Check if it's an authentication error first
        if err.downcast_ref::<AuthenticationFailedError>().is_some() {
            return RPCError {
                kind: RPCErrorKind::AuthenticationFailed,
                underlying: err,
                ignored_files: Vec::new(),
                validation_errors: Vec::new(),
                current_store_version: 0,
            };
        }

        // Try to cast to tonic::Status (Connect RPC errors in Rust are typically tonic::Status)
        if let Some(status) = err.downcast_ref::<Status>() {
            return Self::from_status(status.clone());
        }

        // If it's neither, it's an unknown error
        RPCError {
            kind: RPCErrorKind::Unknown,
            underlying: err,
            ignored_files: Vec::new(),
            validation_errors: Vec::new(),
            current_store_version: 0,
        }
    }

    /// Creates an RPCError from a tonic::Status
    pub fn from_status(status: Status) -> Self {
        let mut rpc_error = RPCError {
            underlying: Box::new(status.clone()),
            ignored_files: Vec::new(),
            validation_errors: Vec::new(),
            kind: RPCErrorKind::Unknown,
            current_store_version: 0,
        };

        match status.code() {
            Code::PermissionDenied => {
                rpc_error.kind = RPCErrorKind::PermissionDenied;
            }
            Code::NotFound => {
                rpc_error.kind = RPCErrorKind::StoreNotFound;
            }
            Code::FailedPrecondition => {
                rpc_error.kind = RPCErrorKind::ConditionUnsatisfied;
            }
            Code::InvalidArgument => {
                // Parse error details from the status
                if let Some((kind, ignored_files, validation_errors)) =
                    Self::parse_error_details(&status)
                {
                    rpc_error.kind = kind;
                    rpc_error.ignored_files = ignored_files;
                    rpc_error.validation_errors = validation_errors;
                } else {
                    rpc_error.kind = RPCErrorKind::InvalidRequest;
                }
            }
            Code::AlreadyExists => {
                // Check for operation discarded details
                if let Some(version) = Self::parse_operation_discarded_details(&status) {
                    rpc_error.kind = RPCErrorKind::OperationDiscarded;
                    rpc_error.current_store_version = version;
                } else {
                    rpc_error.kind = RPCErrorKind::OperationDiscarded;
                }
            }
            _ => {
                rpc_error.kind = RPCErrorKind::Unknown;
            }
        }

        rpc_error
    }

    /// Parses error details from gRPC status for InvalidArgument errors
    fn parse_error_details(status: &Status) -> Option<(RPCErrorKind, Vec<String>, Vec<FileError>)> {
        // Try multiple approaches to parse the error details

        // Approach 1: Parse details directly as protobuf bytes
        let details_bytes = status.details();

        // Try parsing as ErrDetailNoUsableFiles
        if let Ok(no_usable_files) = ErrDetailNoUsableFiles::decode(details_bytes) {
            return Some((
                RPCErrorKind::NoUsableFiles,
                no_usable_files.ignored_files,
                Vec::new(),
            ));
        }

        // Try parsing as ErrDetailValidationFailure
        if let Ok(validation_failure) = ErrDetailValidationFailure::decode(details_bytes) {
            return Some((
                RPCErrorKind::ValidationFailure,
                Vec::new(),
                validation_failure.errors,
            ));
        }

        // Approach 2: Parse as Any message (Google's standard for error details)
        if let Ok(any_message) = Any::decode(details_bytes) {
            return Self::parse_any_error_details(&any_message);
        }

        // Approach 3: Check metadata for error details (some implementations use this)
        Self::parse_error_details_from_metadata(status.metadata())
    }

    /// Parses error details from Any protobuf message
    fn parse_any_error_details(
        any_message: &Any,
    ) -> Option<(RPCErrorKind, Vec<String>, Vec<FileError>)> {
        // Check the type URL to determine which error detail type this is
        if any_message.type_url.contains("ErrDetailNoUsableFiles") {
            if let Ok(no_usable_files) =
                ErrDetailNoUsableFiles::decode(any_message.value.as_slice())
            {
                return Some((
                    RPCErrorKind::NoUsableFiles,
                    no_usable_files.ignored_files,
                    Vec::new(),
                ));
            }
        }

        if any_message.type_url.contains("ErrDetailValidationFailure") {
            if let Ok(validation_failure) =
                ErrDetailValidationFailure::decode(any_message.value.as_slice())
            {
                return Some((
                    RPCErrorKind::ValidationFailure,
                    Vec::new(),
                    validation_failure.errors,
                ));
            }
        }

        None
    }

    /// Parses operation discarded details from gRPC status
    fn parse_operation_discarded_details(status: &Status) -> Option<i64> {
        let details_bytes = status.details();

        // Try direct parsing first
        if let Ok(operation_discarded) = ErrDetailOperationDiscarded::decode(details_bytes) {
            return Some(operation_discarded.current_store_version);
        }

        // Try parsing as Any message
        if let Ok(any_message) = Any::decode(details_bytes) {
            if any_message.type_url.contains("ErrDetailOperationDiscarded") {
                if let Ok(operation_discarded) =
                    ErrDetailOperationDiscarded::decode(any_message.value.as_slice())
                {
                    return Some(operation_discarded.current_store_version);
                }
            }
        }

        None
    }

    /// Parses error details from the status metadata (alternative approach)
    /// Some gRPC implementations put error details in the metadata instead of the status details
    fn parse_error_details_from_metadata(
        metadata: &MetadataMap,
    ) -> Option<(RPCErrorKind, Vec<String>, Vec<FileError>)> {
        // Check for grpc-status-details-bin header which contains serialized error details
        if let Some(details_bin) = metadata.get_bin("grpc-status-details-bin") {
            // The details would be encoded as a google.rpc.Status message
            // For now, we'll skip this implementation as it requires additional complexity
            // In a real implementation, you'd decode the google.rpc.Status and extract the details
        }

        None
    }

    /// Convenience method to check if this is a specific error kind
    pub fn is_kind(&self, kind: RPCErrorKind) -> bool {
        self.kind == kind
    }

    /// Returns true if this error indicates the operation can be retried
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.kind,
            RPCErrorKind::Unknown | RPCErrorKind::ConditionUnsatisfied
        )
    }

    /// Returns true if this error indicates a client-side problem
    pub fn is_client_error(&self) -> bool {
        matches!(
            self.kind,
            RPCErrorKind::AuthenticationFailed
                | RPCErrorKind::InvalidRequest
                | RPCErrorKind::NoUsableFiles
                | RPCErrorKind::PermissionDenied
                | RPCErrorKind::ValidationFailure
        )
    }
}

/// Helper trait to convert various error types to RPCError
pub trait IntoRPCError {
    fn into_rpc_error(self) -> RPCError;
}

impl IntoRPCError for Status {
    fn into_rpc_error(self) -> RPCError {
        RPCError::from_status(self)
    }
}

impl IntoRPCError for Box<dyn Error + Send + Sync> {
    fn into_rpc_error(self) -> RPCError {
        RPCError::new(self)
    }
}

impl From<Status> for RPCError {
    fn from(status: Status) -> Self {
        Self::from_status(status)
    }
}

impl From<AuthenticationFailedError> for RPCError {
    fn from(err: AuthenticationFailedError) -> Self {
        RPCError {
            kind: RPCErrorKind::AuthenticationFailed,
            underlying: Box::new(err),
            ignored_files: Vec::new(),
            validation_errors: Vec::new(),
            current_store_version: 0,
        }
    }
}

/// Client wrapper that converts gRPC errors to RPCError
/*
pub struct StoreClient {
    client: cerbos_store_service_client::CerbosStoreServiceClient<tonic::transport::Channel>,
}
impl StoreClient {
    pub fn new(client: cerbos_store_service_client::CerbosStoreServiceClient<tonic::transport::Channel>) -> Self {
        Self { client }
    }

    pub async fn list_files(&mut self, req: ListFilesRequest) -> Result<ListFilesResponse, RPCError> {
        match self.client.list_files(tonic::Request::new(req)).await {
            Ok(response) => Ok(response.into_inner()),
            Err(status) => Err(status.into_rpc_error()),
        }
    }

    pub async fn get_files(&mut self, req: GetFilesRequest) -> Result<GetFilesResponse, RPCError> {
        match self.client.get_files(tonic::Request::new(req)).await {
            Ok(response) => Ok(response.into_inner()),
            Err(status) => Err(status.into_rpc_error()),
        }
    }

    pub async fn modify_files(&mut self, req: ModifyFilesRequest) -> Result<ModifyFilesResponse, RPCError> {
        match self.client.modify_files(tonic::Request::new(req)).await {
            Ok(response) => Ok(response.into_inner()),
            Err(status) => Err(status.into_rpc_error()),
        }
    }

    pub async fn replace_files(&mut self, req: ReplaceFilesRequest) -> Result<ReplaceFilesResponse, RPCError> {
        match self.client.replace_files(tonic::Request::new(req)).await {
            Ok(response) => Ok(response.into_inner()),
            Err(status) => Err(status.into_rpc_error()),
        }
    }
}
*/
/// Higher-level client with additional convenience methods
/*
pub struct CerbosStoreClient {
    client: StoreClient,
}

impl CerbosStoreClient {
    pub fn new(client: StoreClient) -> Self {
        Self { client }
    }

    /// Replace files with lenient error handling (matches the Go implementation)
    pub async fn replace_files_lenient(
        &mut self,
        req: ReplaceFilesRequest,
    ) -> Result<ReplaceFilesResponse, RPCError> {
        match self.client.replace_files(req).await {
            Ok(response) => Ok(response),
            Err(rpc_err) if rpc_err.kind == RPCErrorKind::OperationDiscarded => {
                // Return a synthetic response with the current store version
                Ok(ReplaceFilesResponse {
                    new_store_version: rpc_err.current_store_version,
                    ignored_files: Vec::new(),
                })
            }
            Err(err) => Err(err),
        }
    }

    /// Modify files with lenient error handling
    pub async fn modify_files_lenient(
        &mut self,
        req: ModifyFilesRequest,
    ) -> Result<ModifyFilesResponse, RPCError> {
        match self.client.modify_files(req).await {
            Ok(response) => Ok(response),
            Err(rpc_err) if rpc_err.kind == RPCErrorKind::OperationDiscarded => {
                // Return a synthetic response with the current store version
                Ok(ModifyFilesResponse {
                    new_store_version: rpc_err.current_store_version,
                })
            }
            Err(err) => Err(err),
        }
    }

    /// Delegate other methods to the underlying client
    pub async fn list_files(
        &mut self,
        req: ListFilesRequest,
    ) -> Result<ListFilesResponse, RPCError> {
        self.client.list_files(req).await
    }

    pub async fn get_files(&mut self, req: GetFilesRequest) -> Result<GetFilesResponse, RPCError> {
        self.client.get_files(req).await
    }

    pub async fn modify_files(
        &mut self,
        req: ModifyFilesRequest,
    ) -> Result<ModifyFilesResponse, RPCError> {
        self.client.modify_files(req).await
    }

    pub async fn replace_files(
        &mut self,
        req: ReplaceFilesRequest,
    ) -> Result<ReplaceFilesResponse, RPCError> {
        self.client.replace_files(req).await
    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    use tonic::{Code, Status};

    #[test]
    fn test_rpc_error_from_permission_denied() {
        let status = Status::new(Code::PermissionDenied, "Access denied");
        let rpc_error = RPCError::from_status(status);

        assert_eq!(rpc_error.kind, RPCErrorKind::PermissionDenied);
        assert!(rpc_error.is_client_error());
        assert!(!rpc_error.is_retryable());
    }

    #[test]
    fn test_rpc_error_from_not_found() {
        let status = Status::new(Code::NotFound, "Store not found");
        let rpc_error = RPCError::from_status(status);

        assert_eq!(rpc_error.kind, RPCErrorKind::StoreNotFound);
        assert!(!rpc_error.is_retryable());
    }

    #[test]
    fn test_rpc_error_from_failed_precondition() {
        let status = Status::new(Code::FailedPrecondition, "Condition not satisfied");
        let rpc_error = RPCError::from_status(status);

        assert_eq!(rpc_error.kind, RPCErrorKind::ConditionUnsatisfied);
        assert!(rpc_error.is_retryable());
    }

    #[test]
    fn test_rpc_error_from_already_exists() {
        let status = Status::new(Code::AlreadyExists, "Operation discarded");
        let rpc_error = RPCError::from_status(status);

        assert_eq!(rpc_error.kind, RPCErrorKind::OperationDiscarded);
    }

    #[test]
    fn test_rpc_error_from_unknown_status() {
        let status = Status::new(Code::Internal, "Internal server error");
        let rpc_error = RPCError::from_status(status);

        assert_eq!(rpc_error.kind, RPCErrorKind::Unknown);
        assert!(rpc_error.is_retryable());
    }

    #[test]
    fn test_authentication_error_conversion() {
        let auth_err = AuthenticationFailedError;
        let rpc_error = RPCError::from(auth_err);

        assert_eq!(rpc_error.kind, RPCErrorKind::AuthenticationFailed);
        assert!(rpc_error.is_client_error());
        assert!(!rpc_error.is_retryable());
    }

    #[test]
    fn test_error_kind_methods() {
        let status = Status::new(Code::InvalidArgument, "Invalid request");
        let rpc_error = RPCError::from_status(status);

        assert!(rpc_error.is_kind(RPCErrorKind::InvalidRequest));
        assert!(!rpc_error.is_kind(RPCErrorKind::PermissionDenied));
        assert!(rpc_error.is_client_error());
    }

    #[test]
    fn test_error_details_with_validation_failure() {
        // This test would require creating a proper Status with encoded error details
        // For now, we'll test the basic structure
        let status = Status::new(Code::InvalidArgument, "Validation failed");
        let rpc_error = RPCError::from_status(status);

        // Without actual error details, this will be classified as InvalidRequest
        assert_eq!(rpc_error.kind, RPCErrorKind::InvalidRequest);
        assert!(rpc_error.validation_errors.is_empty());
        assert!(rpc_error.ignored_files.is_empty());
    }
}
