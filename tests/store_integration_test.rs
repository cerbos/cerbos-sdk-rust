// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
#![cfg(feature = "hub")]

use anyhow::{Context, Result};
use cerbos::genpb::cerbos::cloud::store::v1::GetFilesRequest;
use cerbos::sdk::hub::auth::AuthMiddleware;
use cerbos::sdk::hub::rpc_error::RPCError;
use cerbos::sdk::hub::store::{
    FileFilterBuilder, GetFilesRequestBuilder, ListFilesRequestBuilder, ModifyFilesRequestBuilder,
    ReplaceFilesRequestBuilder, StoreClient,
};
use cerbos::sdk::hub::utils::zip_directory;
use cerbos::sdk::hub::HubClientBuilder;
use std::path::PathBuf;
use std::{env, str};

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
    store_client: StoreClient<AuthMiddleware>,
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
        let test_data_path = get_test_data_path(&["replace_files", "success"]);

        let zip_data = zip_directory(&test_data_path).context("failed to zip data")?;

        let request =
            ReplaceFilesRequestBuilder::new(&self.store_id, "Reset store for test", zip_data)
                .build();

        self.store_client.replace_files_lenient(request).await?;

        self.check_store_has_expected_files().await
    }

    async fn check_store_has_expected_files(&mut self) -> Result<()> {
        let list_request = ListFilesRequestBuilder::new(&self.store_id).build();
        let list_response = self.store_client.list_files(list_request).await?;

        let mut want_files: Vec<String> = WANT_FILES_LIST.iter().map(|s| s.to_string()).collect();
        let mut have_files = list_response.files.clone();
        want_files.sort();
        have_files.sort();

        assert_eq!(want_files, have_files, "Store does not have expected files");
        Ok(())
    }
}

fn get_test_data_path(subpath: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("testdata");
    subpath.iter().for_each(|p| path.push(p));
    path
}
#[tokio::test]
async fn test_auth_error() -> Result<(), Box<dyn std::error::Error>> {
    let api_endpoint = env::var("CERBOS_HUB_API_ENDPOINT")
        .unwrap_or_else(|_| "https://api.cerbos.cloud".to_string());

    let store_id = env::var("CERBOS_HUB_STORE_ID")
        .expect("CERBOS_HUB_STORE_ID environment variable must be set for integration tests");

    let hub_client = HubClientBuilder::new()
        .with_api_endpoint(api_endpoint)
        .with_client_credentials("not a client", "not a secret")
        .build()
        .await?;

    let mut store_client = hub_client.store_client();
    let files = vec!["wibble.yaml".to_string()];
    let request = GetFilesRequest { store_id, files };
    let result = store_client.get_files(request.clone()).await;
    if let Err(RPCError::Unknown {
        message: _,
        underlying: ref e,
    }) = result
    {
        if e.code() == tonic::Code::ResourceExhausted {
            eprintln!("\x1b[91mSkipping test due to too many requests response\x1b[0m");
            return Ok(());
        }
    }
    assert!(
        matches!(
            &result,
            Err(RPCError::AuthenticationFailed {
                message: _,
                underlying: _
            })
        ),
        "{result:?}"
    );
    let result = store_client.get_files(request).await;
    assert!(matches!(
        result,
        Err(RPCError::AuthenticationFailed {
            message: msg,
            underlying: _
        }) if msg.contains("short-circuiting")
    ));
    Ok(())
}
#[tokio::test]
async fn test_replace_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut setup = TestSetup::new().await?;
    setup.reset_store().await?;

    // Test invalid request
    test_replace_files_invalid_request(&mut setup).await?;

    // Test invalid files
    test_replace_files_invalid_files(&mut setup).await?;

    // Test unusable files
    test_replace_files_unusable_files(&mut setup).await?;

    // Test unsuccessful condition
    test_replace_files_unsuccessful_condition(&mut setup).await?;

    Ok(())
}

async fn test_replace_files_invalid_request(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let invalid_zip = b"invalid zip data exceeding 22 bytes in length";
    let request =
        ReplaceFilesRequestBuilder::new(&setup.store_id, "Invalid request", invalid_zip.to_vec())
            .build();

    let result = setup.store_client.replace_files(request).await;
    assert!(matches!(
        result,
        Err(RPCError::InvalidRequest {
            message: _,
            underlying: _
        })
    ));

    Ok(())
}

async fn test_replace_files_invalid_files(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_data_path = get_test_data_path(&["replace_files", "invalid"]);

    let zip_data = zip_directory(&test_data_path)?;
    let request =
        ReplaceFilesRequestBuilder::new(&setup.store_id, "Invalid files test", zip_data).build();

    let result = setup.store_client.replace_files(request).await;
    assert!(
        matches!(result, Err(RPCError::ValidationFailure {message: _, underlying: _, validation_errors}) if validation_errors.len() == 1)
    );

    setup.check_store_has_expected_files().await?;
    Ok(())
}

async fn test_replace_files_unusable_files(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_data_path = get_test_data_path(&["replace_files", "unusable"]);

    let zip_data = zip_directory(&test_data_path)?;
    let request =
        ReplaceFilesRequestBuilder::new(&setup.store_id, "Unusable files test", zip_data).build();

    let result = setup.store_client.replace_files(request).await;
    assert!(
        matches!(result, Err(RPCError::NoUsableFiles {message: _, underlying: _, ignored_files: fs}) if fs.len() == 2 && fs.iter().all(|f| f == ".hidden.yaml" || f == "README.md"))
    );

    Ok(())
}

async fn test_replace_files_unsuccessful_condition(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_data_path = get_test_data_path(&["replace_files", "conditional"]);

    let zip_data = zip_directory(&test_data_path)?;
    let request = ReplaceFilesRequestBuilder::new(&setup.store_id, "Conditional test", zip_data)
        .only_if_version_equals(i64::MAX) // This should fail
        .build();

    let result = setup.store_client.replace_files(request).await;
    assert!(matches!(
        result,
        Err(RPCError::ConditionUnsatisfied {
            message: _,
            underlying: _
        })
    ));

    setup.check_store_has_expected_files().await?;
    Ok(())
}

#[tokio::test]
async fn test_modify_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut setup = TestSetup::new().await?;
    setup.reset_store().await?;

    // Test successful modification
    test_modify_files_success(&mut setup).await?;

    // Test invalid request
    test_modify_files_invalid_request(&mut setup).await?;

    // Test invalid files
    test_modify_files_invalid_files(&mut setup).await?;

    // Test unsuccessful condition
    test_modify_files_unsuccessful_condition(&mut setup).await?;

    Ok(())
}

async fn test_modify_files_success(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let example_content = std::fs::read_to_string(get_test_data_path(&[
        "modify_files",
        "success",
        "example.yaml",
    ]))?;
    let request = ModifyFilesRequestBuilder::new(&setup.store_id, "Test modification")
        .add_or_update_file("example.yaml", example_content.as_bytes().to_vec())
        .build();

    let response = setup.store_client.modify_files(request).await?;
    assert!(response.new_store_version > 0);

    // Verify the file was added
    let get_request = GetFilesRequestBuilder::new(&setup.store_id, vec!["example.yaml"]).build();
    let get_response = setup.store_client.get_files(get_request).await?;

    assert_eq!(get_response.files.len(), 1);

    let retrieved_content = &get_response
        .files
        .iter()
        .find(|&x| x.path == "example.yaml")
        .unwrap()
        .contents;

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
    assert!(matches!(
        result,
        Err(RPCError::InvalidRequest {
            message: _,
            underlying: _
        })
    ));

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
    assert!(
        matches!(result, Err(RPCError::ValidationFailure { message: _, underlying: _, validation_errors }) if validation_errors.len() == 1 )
    );

    Ok(())
}

async fn test_modify_files_unsuccessful_condition(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(get_test_data_path(&[
        "modify_files",
        "conditional",
        "new_policy.yaml",
    ]))?;

    let request = ModifyFilesRequestBuilder::new(&setup.store_id, "Conditional modification")
        .add_or_update_file("new_policy.yaml", content.as_bytes().to_vec())
        .only_if_version_equals(i64::MAX) // This should fail
        .build();

    let result = setup.store_client.modify_files(request).await;

    assert!(
        matches!(
            result,
            Err(RPCError::ConditionUnsatisfied {
                message: _,
                underlying: _
            },)
        ),
        "Expected ConditionUnsatisfied, got: {result:?}"
    );
    Ok(())
}

#[tokio::test]
async fn test_list_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut setup = TestSetup::new().await?;
    setup.reset_store().await?;

    // Test filter with matches
    test_list_files_with_filter_match(&mut setup).await?;

    // Test filter with no matches
    test_list_files_with_no_filter_match(&mut setup).await?;

    Ok(())
}

async fn test_list_files_with_filter_match(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = ListFilesRequestBuilder::new(&setup.store_id)
        .with_file_filter(FileFilterBuilder::path_contains("export_"))
        .build();

    let response = setup.store_client.list_files(request).await?;

    let mut want_files = vec![
        "export_constants/export_constants_01.yaml".to_string(),
        "export_variables/export_variables_01.yaml".to_string(),
    ];

    let mut have_files = response.files.clone();
    have_files.sort();
    want_files.sort();

    assert_eq!(have_files, want_files);

    Ok(())
}

async fn test_list_files_with_no_filter_match(
    setup: &mut TestSetup,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = ListFilesRequestBuilder::new(&setup.store_id)
        .with_file_filter(FileFilterBuilder::path_contains("wibble"))
        .build();

    let response = setup.store_client.list_files(request).await?;
    assert_eq!(response.files.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_get_files() -> Result<(), Box<dyn std::error::Error>> {
    let mut setup = TestSetup::new().await?;
    setup.reset_store().await?;

    // Test non-existent file
    test_get_files_non_existent(&mut setup).await?;

    // Test invalid request
    test_get_files_invalid_request(&mut setup).await?;

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

    assert!(matches!(
        result,
        Err(RPCError::InvalidRequest {
            message: _,
            underlying: _
        })
    ));

    Ok(())
}
