// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use cerbos::sdk::auth::AuthMiddleware;
use cerbos::sdk::hub::HubClientBuilder;
use cerbos::sdk::store::{
    zip_directory, FileFilterBuilder, GetFilesRequestBuilder, GetFilesResponseExt,
    ListFilesRequestBuilder, ModifyFilesRequestBuilder, ReplaceFilesRequestBuilder,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tokio;

// Expected files list from the Go test
const WANT_FILES_LIST: &[&str] = &[
    "_schemas/principal.json",
    "_schemas/resources/leave_request.json",
    "_schemas/resources/purchase_order.json",
    "_schemas/resources/salary_record.json",
    "derived_roles/common_roles.yaml",
    "derived_roles/derived_roles_01.yaml",
    "derived_roles/derived_roles_02.yaml",
    "derived_roles/derived_roles_03.yaml",
    "derived_roles/derived_roles_04.yaml",
    "derived_roles/derived_roles_05.yaml",
    "export_constants/export_constants_01.yaml",
    "export_variables/export_variables_01.yaml",
    "principal_policies/policy_01.yaml",
    "principal_policies/policy_02.yaml",
    "principal_policies/policy_02_acme.hr.yaml",
    "principal_policies/policy_02_acme.sales.yaml",
    "principal_policies/policy_02_acme.yaml",
    "principal_policies/policy_03.yaml",
    "principal_policies/policy_04.yaml",
    "principal_policies/policy_05.yaml",
    "principal_policies/policy_06.yaml",
    "resource_policies/disabled_policy_01.yaml",
    "resource_policies/policy_01.yaml",
    "resource_policies/policy_02.yaml",
    "resource_policies/policy_03.yaml",
    "resource_policies/policy_04.yaml",
    "resource_policies/policy_04_test.yaml",
    "resource_policies/policy_05.yaml",
    "resource_policies/policy_05_acme.hr.uk.brighton.kemptown.yaml",
    "resource_policies/policy_05_acme.hr.uk.brighton.yaml",
    "resource_policies/policy_05_acme.hr.uk.london.yaml",
    "resource_policies/policy_05_acme.hr.uk.yaml",
    "resource_policies/policy_05_acme.hr.yaml",
    "resource_policies/policy_05_acme.yaml",
    "resource_policies/policy_06.yaml",
    "resource_policies/policy_07.yaml",
    "resource_policies/policy_07_acme.yaml",
    "resource_policies/policy_08.yaml",
    "resource_policies/policy_09.yaml",
    "resource_policies/policy_10.yaml",
    "resource_policies/policy_11.yaml",
    "resource_policies/policy_12.yaml",
    "resource_policies/policy_13.yaml",
    "resource_policies/policy_14.yaml",
    "resource_policies/policy_15.yaml",
    "resource_policies/policy_16.yaml",
    "resource_policies/policy_17.acme.sales.yaml",
    "resource_policies/policy_17.acme.yaml",
    "resource_policies/policy_17.yaml",
    "resource_policies/policy_18.yaml",
    "role_policies/policy_01_acme.hr.uk.brighton.yaml",
    "role_policies/policy_02_acme.hr.uk.brighton.yaml",
    "role_policies/policy_03_acme.hr.uk.yaml",
    "role_policies/policy_04_acme.hr.uk.yaml",
    "role_policies/policy_05_acme.hr.uk.london.yaml",
    "role_policies/policy_06_acme.hr.uk.brighton.kemptown.yaml",
    "tests/policy_04_test.yaml",
    "tests/policy_05_test.yaml",
];

struct TestSetup {
    store_client: cerbos::sdk::store::StoreClient<AuthMiddleware>,
    store_id: String,
}

impl TestSetup {
    async fn new() -> Result<Self> {
        let api_endpoint = env::var("CERBOS_HUB_API_ENDPOINT")
            .unwrap_or_else(|_| "https://api.cerbos.cloud".to_string());

        let store_id = env::var("CERBOS_HUB_STORE_ID")
            .expect("CERBOS_HUB_STORE_ID environment variable must be set for integration tests");

        let hub_client = HubClientBuilder::new()
            .with_api_endpoint(api_endpoint)
            .build()
            .await?;

        let store_client = hub_client.store_client();

        Ok(TestSetup {
            store_client,
            store_id,
        })
    }

    async fn reset_store(&mut self) -> Result<()> {
        let test_data_path = get_test_data_path("replace_files/success");
        println!("test_data_path {:?}", test_data_path);

        let zip_data = zip_directory(&test_data_path)?;

        let request =
            ReplaceFilesRequestBuilder::new(&self.store_id, "Reset store for test", zip_data)
                .build();

        let response = self.store_client.replace_files(request).await?;
        assert!(response.new_store_version > 0);

        // Verify the reset worked
        let list_request = ListFilesRequestBuilder::new(&self.store_id).build();
        let list_response = self.store_client.list_files(list_request).await?;

        let mut want_files: Vec<String> = WANT_FILES_LIST.iter().map(|s| s.to_string()).collect();
        let mut have_files = list_response.files.clone();
        want_files.sort();
        have_files.sort();

        assert_eq!(
            want_files, have_files,
            "Store reset did not produce expected files"
        );

        Ok(())
    }
}

fn get_test_data_path(subpath: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("testdata");
    path.push(subpath);
    path
}

#[tokio::test]
#[ignore]
async fn test_store_integration() -> Result<(), Box<dyn std::error::Error>> {
    let mut setup = TestSetup::new().await?;

    test_replace_files(&mut setup).await?;
    // test_modify_files(&mut setup).await?;
    // test_list_files(&mut setup).await?;
    // test_get_files(&mut setup).await?;

    Ok(())
}

async fn test_replace_files(setup: &mut TestSetup) -> Result<(), Box<dyn std::error::Error>> {
    setup.reset_store().await?;

    // Test invalid request
    test_replace_files_invalid_request(setup).await?;

    // Test invalid files
    test_replace_files_invalid_files(setup).await?;

    // Test unusable files
    test_replace_files_unusable_files(setup).await?;

    // Test unsuccessful condition
    test_replace_files_unsuccessful_condition(setup).await?;

    Ok(())
}

async fn test_replace_files_invalid_request(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let invalid_zip = b"invalid zip data";
    let request =
        ReplaceFilesRequestBuilder::new(&setup.store_id, "Invalid request", invalid_zip.to_vec())
            .build();

    let result = setup.store_client.replace_files(request).await;
    assert!(result.is_err(), "Expected error for invalid zip data");

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("validation") || error_msg.contains("invalid"),
        "Expected validation error, got: {}",
        error_msg
    );

    Ok(())
}

async fn test_replace_files_invalid_files(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_data_path = get_test_data_path("replace_files/invalid");
    if !test_data_path.exists() {
        // Create test data if it doesn't exist
        create_invalid_test_data(&test_data_path)?;
    }

    let zip_data = zip_directory(&test_data_path)?;
    let request =
        ReplaceFilesRequestBuilder::new(&setup.store_id, "Invalid files test", zip_data).build();

    let result = setup.store_client.replace_files(request).await;
    assert!(result.is_err(), "Expected error for invalid files");

    // Verify store wasn't changed
    let list_request = ListFilesRequestBuilder::new(&setup.store_id).build();
    let list_response = setup.store_client.list_files(list_request).await?;

    let mut want_files: Vec<String> = WANT_FILES_LIST.iter().map(|s| s.to_string()).collect();
    let mut have_files = list_response.files.clone();
    want_files.sort();
    have_files.sort();

    assert_eq!(
        want_files, have_files,
        "Store should not have changed after invalid files"
    );

    Ok(())
}

async fn test_replace_files_unusable_files(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_data_path = get_test_data_path("replace_files/unusable");
    if !test_data_path.exists() {
        create_unusable_test_data(&test_data_path)?;
    }

    let zip_data = zip_directory(&test_data_path)?;
    let request =
        ReplaceFilesRequestBuilder::new(&setup.store_id, "Unusable files test", zip_data).build();

    let result = setup.store_client.replace_files(request).await;
    assert!(result.is_err(), "Expected error for unusable files");

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("no usable files") || error_msg.contains("ignored"),
        "Expected no usable files error, got: {}",
        error_msg
    );

    Ok(())
}

async fn test_replace_files_unsuccessful_condition(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_data_path = get_test_data_path("replace_files/conditional");
    if !test_data_path.exists() {
        create_conditional_test_data(&test_data_path)?;
    }

    let zip_data = zip_directory(&test_data_path)?;
    let request = ReplaceFilesRequestBuilder::new(&setup.store_id, "Conditional test", zip_data)
        .only_if_version_equals(i64::MAX) // This should fail
        .build();

    let result = setup.store_client.replace_files(request).await;
    assert!(result.is_err(), "Expected error for unsuccessful condition");

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("condition") || error_msg.contains("version"),
        "Expected condition error, got: {}",
        error_msg
    );

    // Verify store wasn't changed
    let list_request = ListFilesRequestBuilder::new(&setup.store_id).build();
    let list_response = setup.store_client.list_files(list_request).await?;

    let mut want_files: Vec<String> = WANT_FILES_LIST.iter().map(|s| s.to_string()).collect();
    let mut have_files = list_response.files.clone();
    want_files.sort();
    have_files.sort();

    assert_eq!(
        want_files, have_files,
        "Store should not have changed after failed condition"
    );

    Ok(())
}

async fn test_modify_files(setup: &mut TestSetup) -> Result<(), Box<dyn std::error::Error>> {
    setup.reset_store().await?;

    // Test successful modification
    test_modify_files_success(setup).await?;

    // Test invalid request
    test_modify_files_invalid_request(setup).await?;

    // Test invalid files
    test_modify_files_invalid_files(setup).await?;

    // Test unsuccessful condition
    test_modify_files_unsuccessful_condition(setup).await?;

    Ok(())
}

async fn test_modify_files_success(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let example_content = r#"
apiVersion: api.cerbos.dev/v1
resourcePolicy:
  version: "default"
  resource: "example"
  rules:
    - actions: ["read"]
      effect: EFFECT_ALLOW
      roles: ["user"]
"#;

    let request = ModifyFilesRequestBuilder::new(&setup.store_id, "Test modification")
        .add_or_update_file("example.yaml", example_content.as_bytes().to_vec())
        .build();

    let response = setup.store_client.modify_files(request).await?;
    assert!(response.new_store_version > 0);

    // Verify the file was added
    let get_request = GetFilesRequestBuilder::new(&setup.store_id, vec!["example.yaml"]).build();
    let get_response = setup.store_client.get_files(get_request).await?;

    let file_map = get_response.as_map();
    assert_eq!(file_map.len(), 1);

    let retrieved_content = file_map.get("example.yaml").unwrap();
    assert_eq!(
        std::str::from_utf8(retrieved_content)?.trim(),
        example_content.trim()
    );

    Ok(())
}

async fn test_modify_files_invalid_request(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    // Empty operations should be invalid
    let request = ModifyFilesRequestBuilder::new(&setup.store_id, "Empty modification").build();

    let result = setup.store_client.modify_files(request).await;
    assert!(result.is_err(), "Expected error for empty operations");

    Ok(())
}

async fn test_modify_files_invalid_files(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let invalid_content = "invalid yaml content: [[[";

    let request = ModifyFilesRequestBuilder::new(&setup.store_id, "Invalid file modification")
        .add_or_update_file("invalid.yaml", invalid_content.as_bytes().to_vec())
        .build();

    let result = setup.store_client.modify_files(request).await;
    assert!(result.is_err(), "Expected error for invalid file content");

    Ok(())
}

async fn test_modify_files_unsuccessful_condition(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = "test: content";

    let request = ModifyFilesRequestBuilder::new(&setup.store_id, "Conditional modification")
        .add_or_update_file("conditional.yaml", content.as_bytes().to_vec())
        .only_if_version_equals(i64::MAX) // This should fail
        .build();

    let result = setup.store_client.modify_files(request).await;
    assert!(result.is_err(), "Expected error for unsuccessful condition");

    Ok(())
}

async fn test_list_files(setup: &mut TestSetup) -> Result<(), Box<dyn std::error::Error>> {
    setup.reset_store().await?;

    // Test filter with matches
    test_list_files_with_filter_match(setup).await?;

    // Test filter with no matches
    test_list_files_with_no_filter_match(setup).await?;

    Ok(())
}

async fn test_list_files_with_filter_match(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = ListFilesRequestBuilder::new(&setup.store_id)
        .with_file_filter(FileFilterBuilder::path_like("export_"))
        .build();

    let response = setup.store_client.list_files(request).await?;

    let expected_files = vec![
        "export_constants/export_constants_01.yaml",
        "export_variables/export_variables_01.yaml",
    ];

    let mut have_files = response.files.clone();
    have_files.sort();
    let mut want_files = expected_files
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    want_files.sort();

    assert_eq!(have_files, want_files);

    Ok(())
}

async fn test_list_files_with_no_filter_match(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = ListFilesRequestBuilder::new(&setup.store_id)
        .with_file_filter(FileFilterBuilder::path_like("wibble"))
        .build();

    let response = setup.store_client.list_files(request).await?;
    assert_eq!(response.files.len(), 0);

    Ok(())
}

async fn test_get_files(setup: &mut TestSetup) -> Result<(), Box<dyn std::error::Error>> {
    setup.reset_store().await?;

    // Test non-existent file
    test_get_files_non_existent(setup).await?;

    // Test invalid request
    test_get_files_invalid_request(setup).await?;

    Ok(())
}

async fn test_get_files_non_existent(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = GetFilesRequestBuilder::new(&setup.store_id, vec!["wibble.yaml"]).build();
    let response = setup.store_client.get_files(request).await?;

    assert_eq!(response.files.len(), 0);

    Ok(())
}

async fn test_get_files_invalid_request(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = GetFilesRequestBuilder::new(&setup.store_id, Vec::<String>::new()).build();
    let result = setup.store_client.get_files(request).await;

    assert!(result.is_err(), "Expected error for empty file list");

    Ok(())
}

// Helper functions to create test data

fn create_invalid_test_data(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path)?;

    // Create an invalid policy file
    let invalid_policy = r#"
apiVersion: api.cerbos.dev/v1
resourcePolicy:
  version: "default"
  resource: ""  # Empty resource should be invalid
  rules: []
"#;

    fs::write(path.join("invalid_policy.yaml"), invalid_policy)?;
    Ok(())
}

fn create_unusable_test_data(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path)?;

    // Create files that should be ignored
    fs::write(path.join(".hidden.yaml"), "hidden: file")?;
    fs::write(path.join("README.md"), "# This is a readme file")?;

    Ok(())
}

fn create_conditional_test_data(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path)?;

    let policy = r#"
apiVersion: api.cerbos.dev/v1
resourcePolicy:
  version: "default"
  resource: "conditional_test"
  rules:
    - actions: ["read"]
      effect: EFFECT_ALLOW
      roles: ["user"]
"#;

    fs::write(path.join("conditional_policy.yaml"), policy)?;
    Ok(())
}

fn create_success_test_data(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path)?;

    // Create the full set of expected files for testing
    for file_path in WANT_FILES_LIST {
        let full_path = path.join(file_path);

        // Create parent directories
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Create appropriate content based on file type
        let content = if file_path.ends_with(".json") {
            create_json_schema_content(file_path)
        } else {
            create_yaml_policy_content(file_path)
        };

        fs::write(full_path, content)?;
    }

    Ok(())
}

fn create_json_schema_content(file_path: &str) -> String {
    match file_path {
        "_schemas/principal.json" => r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "id": {"type": "string"},
    "roles": {"type": "array", "items": {"type": "string"}}
  }
}"#
        .to_string(),
        _ => r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "id": {"type": "string"}
  }
}"#
        .to_string(),
    }
}

fn create_yaml_policy_content(file_path: &str) -> String {
    if file_path.starts_with("resource_policies/") {
        let resource_name = file_path
            .strip_prefix("resource_policies/")
            .unwrap_or("resource")
            .strip_suffix(".yaml")
            .unwrap_or("resource");

        format!(
            r#"
apiVersion: api.cerbos.dev/v1
resourcePolicy:
  version: "default"
  resource: "{}"
  rules:
    - actions: ["read"]
      effect: EFFECT_ALLOW
      roles: ["user"]
"#,
            resource_name
        )
    } else if file_path.starts_with("principal_policies/") {
        format!(
            r#"
apiVersion: api.cerbos.dev/v1
principalPolicy:
  version: "default"
  principal: "user"
  rules:
    - resource: "document"
      actions:
        - action: "read"
          effect: EFFECT_ALLOW
"#
        )
    } else if file_path.starts_with("derived_roles/") {
        format!(
            r#"
apiVersion: api.cerbos.dev/v1
derivedRoles:
  name: "common_roles"
  definitions:
    - name: "owner"
      parentRoles: ["user"]
      condition:
        match:
          expr: "resource.attr.owner == principal.id"
"#
        )
    } else {
        format!(
            r#"
# Test file: {}
test: content
"#,
            file_path
        )
    }
}

// Helper to set up test data directory if needed
#[tokio::test]
#[ignore]
async fn setup_test_data() -> Result<(), Box<dyn std::error::Error>> {
    let test_data_root = get_test_data_path("");

    // Create success test data
    let success_path = test_data_root.join("replace_files").join("success");
    if !success_path.exists() {
        create_success_test_data(&success_path)?;
        println!("Created success test data at: {:?}", success_path);
    }

    // Create other test data directories
    let invalid_path = test_data_root.join("replace_files").join("invalid");
    create_invalid_test_data(&invalid_path)?;

    let unusable_path = test_data_root.join("replace_files").join("unusable");
    create_unusable_test_data(&unusable_path)?;

    let conditional_path = test_data_root.join("replace_files").join("conditional");
    create_conditional_test_data(&conditional_path)?;

    println!("Test data setup complete");
    Ok(())
}
