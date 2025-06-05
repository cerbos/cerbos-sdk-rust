// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::io::{Cursor, Read, Write};
use std::{collections::HashMap, path::Path};

use crate::genpb::cerbos::cloud::store::v1::{
    cerbos_store_service_client::CerbosStoreServiceClient,
    change_details::{Git, Internal, Origin, Uploader},
    file_op::Op,
    modify_files_request::Condition as ModifyCondition,
    replace_files_request::Condition as ReplaceCondition,
    string_match::{InList, Match},
    ChangeDetails, File, FileFilter, FileOp, GetFilesRequest, GetFilesResponse, ListFilesRequest,
    ListFilesResponse, ModifyFilesRequest, ModifyFilesResponse, ReplaceFilesRequest,
    ReplaceFilesResponse, StringMatch,
};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

/// Store client for interacting with Cerbos Hub file store
pub struct StoreClient<T> {
    client: CerbosStoreServiceClient<T>,
}

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
impl<T> StoreClient<T>
where
    T: tonic::client::GrpcService<tonic::body::Body>,
    T::Error: Into<StdError>,
    T::ResponseBody: http_body::Body<Data = prost::bytes::Bytes> + std::marker::Send + 'static,
    <T::ResponseBody as http_body::Body>::Error: Into<StdError> + std::marker::Send,
{
    pub fn new(channel: T) -> Self {
        Self {
            client: CerbosStoreServiceClient::new(channel),
        }
    }

    /// Replace all files in the store with the provided zip content
    pub async fn replace_files(
        &mut self,
        request: ReplaceFilesRequest,
    ) -> Result<ReplaceFilesResponse, tonic::Status> {
        let response = self.client.replace_files(request).await?;
        Ok(response.into_inner())
    }

    /// Modify specific files in the store
    pub async fn modify_files(
        &mut self,
        request: ModifyFilesRequest,
    ) -> Result<ModifyFilesResponse, tonic::Status> {
        let response = self.client.modify_files(request).await?;

        Ok(response.into_inner())
    }

    /// List files in the store
    pub async fn list_files(
        &mut self,
        request: ListFilesRequest,
    ) -> Result<ListFilesResponse, tonic::Status> {
        let response = self.client.list_files(request).await?;

        Ok(response.into_inner())
    }

    /// Get specific files from the store
    pub async fn get_files(
        &mut self,
        request: GetFilesRequest,
    ) -> Result<GetFilesResponse, tonic::Status> {
        let response = self.client.get_files(request).await?;

        Ok(response.into_inner())
    }
}

/// Builder for ModifyFilesRequest
#[derive(Debug, Clone)]
pub struct ModifyFilesRequestBuilder {
    store_id: String,
    condition: Option<ModifyCondition>,
    operations: Vec<FileOp>,
    change_details: Option<ChangeDetails>,
}

impl ModifyFilesRequestBuilder {
    pub fn new(store_id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            store_id: store_id.into(),
            condition: None,
            operations: Vec::new(),
            change_details: Some(ChangeDetailsBuilder::new(description).build()),
        }
    }

    pub fn add_or_update_file(mut self, path: impl Into<String>, contents: Vec<u8>) -> Self {
        self.operations.push(FileOp {
            op: Some(Op::AddOrUpdate(File {
                path: path.into(),
                contents,
            })),
        });
        self
    }

    pub fn delete_file(mut self, path: impl Into<String>) -> Self {
        self.operations.push(FileOp {
            op: Some(Op::Delete(path.into())),
        });
        self
    }

    pub fn add_operations<I>(mut self, ops: I) -> Self
    where
        I: IntoIterator<Item = FileOp>,
    {
        self.operations.extend(ops);
        self
    }

    pub fn only_if_version_equals(mut self, version: i64) -> Self {
        self.condition = Some(ModifyCondition {
            store_version_must_equal: version,
        });
        self
    }

    pub fn with_change_details(mut self, change_details: ChangeDetails) -> Self {
        self.change_details = Some(change_details);
        self
    }

    pub fn build(self) -> ModifyFilesRequest {
        ModifyFilesRequest {
            store_id: self.store_id,
            condition: self.condition,
            operations: self.operations,
            change_details: self.change_details,
        }
    }
}

/// Builder for ReplaceFilesRequest
#[derive(Debug, Clone)]
pub struct ReplaceFilesRequestBuilder {
    store_id: String,
    condition: Option<ReplaceCondition>,
    zipped_contents: Vec<u8>,
    change_details: Option<ChangeDetails>,
}

impl ReplaceFilesRequestBuilder {
    pub fn new(
        store_id: impl Into<String>,
        description: impl Into<String>,
        zipped_contents: Vec<u8>,
    ) -> Self {
        Self {
            store_id: store_id.into(),
            condition: None,
            zipped_contents,
            change_details: Some(
                ChangeDetailsBuilder::new(description)
                    .with_origin_internal("test")
                    .build(),
            ),
        }
    }

    pub fn only_if_version_equals(mut self, version: i64) -> Self {
        self.condition = Some(ReplaceCondition {
            store_version_must_equal: version,
        });
        self
    }

    pub fn with_change_details(mut self, change_details: ChangeDetails) -> Self {
        self.change_details = Some(change_details);
        self
    }

    pub fn build(self) -> ReplaceFilesRequest {
        ReplaceFilesRequest {
            store_id: self.store_id,
            condition: self.condition,
            zipped_contents: self.zipped_contents,
            change_details: self.change_details,
        }
    }
}

/// Builder for ListFilesRequest
#[derive(Debug, Clone)]
pub struct ListFilesRequestBuilder {
    store_id: String,
    filter: Option<FileFilter>,
}

impl ListFilesRequestBuilder {
    pub fn new(store_id: impl Into<String>) -> Self {
        Self {
            store_id: store_id.into(),
            filter: None,
        }
    }

    pub fn with_file_filter(mut self, filter: FileFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn build(self) -> ListFilesRequest {
        ListFilesRequest {
            store_id: self.store_id,
            filter: self.filter,
        }
    }
}

/// Builder for GetFilesRequest
#[derive(Debug, Clone)]
pub struct GetFilesRequestBuilder {
    store_id: String,
    files: Vec<String>,
}

impl GetFilesRequestBuilder {
    pub fn new<I, S>(store_id: impl Into<String>, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            store_id: store_id.into(),
            files: files.into_iter().map(Into::into).collect(),
        }
    }

    pub fn build(self) -> GetFilesRequest {
        GetFilesRequest {
            store_id: self.store_id,
            files: self.files,
        }
    }
}

/// Builder for ChangeDetails
#[derive(Debug, Clone)]
pub struct ChangeDetailsBuilder {
    description: String,
    uploader: Option<Uploader>,
    origin: Option<Origin>,
}

impl ChangeDetailsBuilder {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            uploader: Some(Uploader {
                name: "cerbos-sdk-rust".to_string(),
                metadata: HashMap::new(),
            }),
            origin: None,
        }
    }

    pub fn with_uploader(mut self, name: impl Into<String>) -> Self {
        if let Some(ref mut uploader) = self.uploader {
            uploader.name = name.into();
        } else {
            self.uploader = Some(Uploader {
                name: name.into(),
                metadata: HashMap::new(),
            });
        }
        self
    }

    pub fn with_uploader_details(mut self, uploader: Uploader) -> Self {
        self.uploader = Some(uploader);
        self
    }

    pub fn with_origin_git(mut self, repo: impl Into<String>, hash: impl Into<String>) -> Self {
        self.origin = Some(Origin::Git(Git {
            repo: repo.into(),
            r#ref: String::new(),
            hash: hash.into(),
            message: String::new(),
            committer: String::new(),
            commit_date: None,
            author: String::new(),
            author_date: None,
        }));
        self
    }

    pub fn with_origin_git_details(mut self, git_info: Git) -> Self {
        self.origin = Some(Origin::Git(git_info));
        self
    }

    pub fn with_origin_internal(mut self, source: impl Into<String>) -> Self {
        self.origin = Some(Origin::Internal(Internal {
            source: source.into(),
            metadata: HashMap::new(),
        }));
        self
    }

    pub fn with_origin_internal_details(mut self, internal_info: Internal) -> Self {
        self.origin = Some(Origin::Internal(internal_info));
        self
    }

    pub fn build(self) -> ChangeDetails {
        ChangeDetails {
            description: self.description,
            uploader: self.uploader,
            origin: self.origin,
        }
    }
}

/// File filter utilities
pub struct FileFilterBuilder;

impl FileFilterBuilder {
    /// Create a filter that matches the given path exactly
    pub fn path_equals(path: impl Into<String>) -> FileFilter {
        FileFilter {
            path: Some(StringMatch {
                r#match: Some(Match::Equals(path.into())),
            }),
        }
    }

    /// Create a filter that matches one or more of the set of paths exactly
    pub fn path_in<I, S>(paths: I) -> FileFilter
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        FileFilter {
            path: Some(StringMatch {
                r#match: Some(Match::In(InList {
                    values: paths.into_iter().map(Into::into).collect(),
                })),
            }),
        }
    }

    /// Create a filter that partially matches the given path
    pub fn path_like(path: impl Into<String>) -> FileFilter {
        FileFilter {
            path: Some(StringMatch {
                r#match: Some(Match::Like(path.into())),
            }),
        }
    }
}

/// Utility function to create zipped data from a directory
pub fn zip_directory(dir_path: &std::path::Path) -> anyhow::Result<Vec<u8>> {
    let mut buffer = Vec::new();

    let walkdir = WalkDir::new(dir_path);
    let it = walkdir.into_iter();

    let cursor = Cursor::new(&mut buffer);
    let mut zip = zip::ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::STORE);

    let prefix = Path::new(dir_path);
    let mut file_buffer = Vec::new();
    for entry in it.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = std::fs::File::open(path)?;

            f.read_to_end(&mut file_buffer)?;
            zip.write_all(&file_buffer)?;
            file_buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;

    Ok(buffer)
}

/// Helper to convert metadata map to protobuf Value map
// pub fn to_metadata(metadata: HashMap<String, serde_json::Value>) -> Result<HashMap<String, Value>> {
//     let mut result = HashMap::new();
//     for (key, value) in metadata {
//         let pb_value = json_to_protobuf_value(value)?;
//         result.insert(key, pb_value);
//     }
//     Ok(result)
// }

// fn json_to_protobuf_value(value: serde_json::Value) -> Result<Value> {
//     let kind = match value {
//         serde_json::Value::Null => Some(Kind::NullValue(0)),
//         serde_json::Value::Bool(b) => Some(Kind::BoolValue(b)),
//         serde_json::Value::Number(n) => {
//             if let Some(f) = n.as_f64() {
//                 Some(Kind::NumberValue(f))
//             } else {
//                 return Err(anyhow::anyhow!("Invalid number value"));
//             }
//         }
//         serde_json::Value::String(s) => Some(Kind::StringValue(s)),
//         serde_json::Value::Array(arr) => {
//             let values: Result<Vec<Value>> = arr.into_iter().map(json_to_protobuf_value).collect();
//             Some(Kind::ListValue(prost_types::ListValue { values: values? }))
//         }
//         serde_json::Value::Object(obj) => {
//             let mut fields = HashMap::new();
//             for (k, v) in obj {
//                 fields.insert(k, json_to_protobuf_value(v)?);
//             }
//             Some(Kind::StructValue(Struct { fields }))
//         }
//     };

//     Ok(Value { kind })
// }

/// Extension trait for GetFilesResponse to provide convenient access methods
pub trait GetFilesResponseExt {
    fn as_map(&self) -> HashMap<String, &[u8]>;
}

impl GetFilesResponseExt for GetFilesResponse {
    fn as_map(&self) -> HashMap<String, &[u8]> {
        let mut map = HashMap::new();
        for file in &self.files {
            map.insert(file.path.clone(), file.contents.as_slice());
        }
        map
    }
}
