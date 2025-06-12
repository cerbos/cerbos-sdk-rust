// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

use prost::Message;
use prost_types::Any;
use tonic::Status;

use crate::genpb::{
    cerbos::cloud::store::v1::{
        ErrDetailNoUsableFiles, ErrDetailOperationDiscarded, ErrDetailValidationFailure, FileError,
    },
    google::rpc::Status as GoogleStatus,
};

#[derive(Error, Debug, Clone)]
pub enum RPCError {
    #[error("{message:?}")]
    AuthenticationFailed {
        message: String,
        underlying: tonic::Status,
    },
    #[error("{message:?}")]
    ConditionUnsatisfied {
        message: String,
        underlying: tonic::Status,
    },
    #[error("{message:?}")]
    InvalidRequest {
        message: String,
        underlying: tonic::Status,
    },
    #[error("{message:?}")]
    NoUsableFiles {
        message: String,
        underlying: tonic::Status,
        ignored_files: Vec<String>,
    },
    #[error("{message:?}")]
    OperationDiscarded {
        message: String,
        underlying: tonic::Status,
        current_store_version: i64,
    },
    #[error("{message:?}")]
    PermissionDenied {
        message: String,
        underlying: tonic::Status,
    },
    #[error("{message:?}")]
    StoreNotFound {
        message: String,
        underlying: tonic::Status,
    },
    #[error("{message:?}")]
    Unknown {
        message: String,
        underlying: tonic::Status,
    },
    #[error("{message:?}")]
    ValidationFailure {
        message: String,
        underlying: tonic::Status,
        validation_errors: Vec<FileError>,
    },
    #[error("{message:?}")]
    ClientSideValidationError { message: String },
}
trait TypeUrl {
    fn type_url() -> &'static str;
}
impl TypeUrl for ErrDetailValidationFailure {
    fn type_url() -> &'static str {
        "type.googleapis.com/cerbos.cloud.store.v1.ErrDetailValidationFailure"
    }
}
impl TypeUrl for ErrDetailNoUsableFiles {
    fn type_url() -> &'static str {
        "type.googleapis.com/cerbos.cloud.store.v1.ErrDetailNoUsableFiles"
    }
}
impl TypeUrl for ErrDetailOperationDiscarded {
    fn type_url() -> &'static str {
        "type.googleapis.com/cerbos.cloud.store.v1.ErrDetailOperationDiscarded"
    }
}
impl From<tonic::Status> for RPCError {
    fn from(status: Status) -> Self {
        match status.code() {
            tonic::Code::PermissionDenied => RPCError::PermissionDenied {
                message: status.message().to_string(),
                underlying: status,
            },
            tonic::Code::NotFound => RPCError::StoreNotFound {
                message: status.message().to_string(),
                underlying: status,
            },
            tonic::Code::FailedPrecondition => RPCError::ConditionUnsatisfied {
                message: status.message().to_string(),
                underlying: status,
            },
            tonic::Code::InvalidArgument => {
                if let Ok(google_status) = GoogleStatus::decode(status.details()) {
                    for Any { type_url, value } in google_status.details {
                        if type_url == ErrDetailValidationFailure::type_url() {
                            if let Ok(inner) = ErrDetailValidationFailure::decode(value.as_slice())
                            {
                                return RPCError::ValidationFailure {
                                    message: status.message().to_string(),
                                    underlying: status,
                                    validation_errors: inner.errors,
                                };
                            }
                        } else if type_url == ErrDetailNoUsableFiles::type_url() {
                            if let Ok(inner) = ErrDetailNoUsableFiles::decode(value.as_slice()) {
                                return RPCError::NoUsableFiles {
                                    message: status.message().to_string(),
                                    underlying: status,
                                    ignored_files: inner.ignored_files,
                                };
                            }
                        }
                    }
                }
                return RPCError::InvalidRequest {
                    message: status.message().to_string(),
                    underlying: status,
                };
            }
            tonic::Code::AlreadyExists => {
                if let Ok(google_status) = GoogleStatus::decode(status.details()) {
                    for Any { type_url, value } in google_status.details {
                        if type_url == ErrDetailOperationDiscarded::type_url() {
                            if let Ok(inner) = ErrDetailOperationDiscarded::decode(value.as_slice())
                            {
                                return RPCError::OperationDiscarded {
                                    message: status.message().to_string(),
                                    underlying: status,
                                    current_store_version: inner.current_store_version,
                                };
                            }
                        }
                    }
                }
                return RPCError::OperationDiscarded {
                    message: status.message().to_string(),
                    underlying: status,
                    current_store_version: 0,
                };
            }
            tonic::Code::Unauthenticated => RPCError::AuthenticationFailed {
                message: status.message().to_string(),
                underlying: status,
            },
            _ => RPCError::Unknown {
                message: status.message().to_string(),
                underlying: status,
            },
        }
    }
}
