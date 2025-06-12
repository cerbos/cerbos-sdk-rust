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

/*
fn parse_any_detail(any_detail: &Any, parsed_details: &mut ParsedErrorDetails) {
    match any_detail.type_url.as_str() {
        "type.googleapis.com/cerbos.cloud.store.v1.ErrDetailValidationFailure" => {
            if let Ok(validation_failure) =
                ErrDetailValidationFailure::decode(any_detail.value.as_slice())
            {
                parsed_details
                    .validation_failures
                    .extend(validation_failure.errors);
            }
        }
        "type.googleapis.com/cerbos.cloud.store.v1.ErrDetailNoUsableFiles" => {
            if let Ok(no_usable_files) = ErrDetailNoUsableFiles::decode(any_detail.value.as_slice())
            {
                parsed_details
                    .ignored_files
                    .extend(no_usable_files.ignored_files);
            }
        }
        "type.googleapis.com/cerbos.cloud.store.v1.ErrDetailOperationDiscarded" => {
            if let Ok(operation_discarded) =
                ErrDetailOperationDiscarded::decode(any_detail.value.as_slice())
            {
                parsed_details.current_store_version =
                    Some(operation_discarded.current_store_version);
            }
        }
        _ => {
            // Unknown type, log or handle as needed
            eprintln!("Unknown error detail type: {}", any_detail.type_url);
        }
    }
}
*/

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
                let copy = status.clone();
                if let Ok(google_status) = GoogleStatus::decode(copy.details()) {
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

// Example usage with your specific error
#[cfg(test)]
mod tests {
    use super::*;
    use tonic::{Code, Status};

    #[test]
    fn test_parse_validation_failure() {
        // Your actual error details bytes
        let details_bytes = b"\x08\x03\x12\x12validation failure\x1a\x8e\x01\nDtype.googleapis.com/cerbos.cloud.store.v1.ErrDetailValidationFailure\x12F\nD\n\x0fbad_policy.yaml\x10\x03\x1a/failed to read policy: 1:1 unknown field \"this\"";

        // Create a status with these details
        let status = Status::with_details(
            Code::InvalidArgument,
            "validation failure",
            details_bytes.to_vec().into(),
        );

        let rpc_error = RPCError::from_status(status);

        assert_eq!(rpc_error.kind, RPCError::ValidationFailure);
        assert_eq!(rpc_error.validation_errors.len(), 1);

        let validation_error = &rpc_error.validation_errors[0];
        assert_eq!(validation_error.file, "bad_policy.yaml");
        assert_eq!(validation_error.cause, 3); // InvalidFileContents
        assert_eq!(
            validation_error.details,
            "failed to read policy: 1:1 unknown field \"this\""
        );

        println!("Parsed validation error: {:?}", validation_error);
    }

    #[test]
    fn test_manual_parsing() {
        // Manual test of the parsing logic
        let details_bytes = b"\x08\x03\x12\x12validation failure\x1a\x8e\x01\nDtype.googleapis.com/cerbos.cloud.store.v1.ErrDetailValidationFailure\x12F\nD\n\x0fbad_policy.yaml\x10\x03\x1a/failed to read policy: 1:1 unknown field \"this\"";

        // Try to parse as google.rpc.Status
        if let Ok(google_status) = GoogleStatus::decode(&details_bytes[..]) {
            println!("Parsed google.rpc.Status:");
            println!("  Code: {}", google_status.code);
            println!("  Message: {}", google_status.message);
            println!("  Details count: {}", google_status.details.len());

            for (i, detail) in google_status.details.iter().enumerate() {
                println!("  Detail {}: type_url = {}", i, detail.type_url);

                if detail.type_url
                    == "type.googleapis.com/cerbos.cloud.store.v1.ErrDetailValidationFailure"
                {
                    if let Ok(validation_failure) =
                        ErrDetailValidationFailure::decode(detail.value.as_slice())
                    {
                        println!("    Validation errors: {}", validation_failure.errors.len());
                        for (j, error) in validation_failure.errors.iter().enumerate() {
                            println!(
                                "      Error {}: file={}, cause={}, details={}",
                                j, error.file, error.cause, error.details
                            );
                        }
                    }
                }
            }
        } else {
            println!("Failed to parse as google.rpc.Status");
        }
    }
}

// Helper function to create a properly formatted Status with details
pub fn create_status_with_details(code: tonic::Code, message: &str, details: Vec<Any>) -> Status {
    let google_status = GoogleStatus {
        code: code as i32,
        message: message.to_string(),
        details,
    };

    let mut buf = Vec::new();
    google_status.encode(&mut buf).unwrap();

    Status::with_details(code, message, buf.into())
}
