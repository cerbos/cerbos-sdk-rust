// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use anyhow::Result;
use cerbos::{
    genpb::{cerbos::policy, google},
    sdk::admin::model::{FilterOptions, PolicySet},
};

const ADMIN_USERNAME: &'static str = "cerbos";
const ADMIN_PASSWORD: &'static str = "cerbosAdmin";

#[cfg(feature = "testcontainers")]
trait Stoppable {
    async fn stop(&self) -> anyhow::Result<()>;
}

#[cfg(feature = "testcontainers")]
impl<T: testcontainers::Image> Stoppable for testcontainers::ContainerAsync<T> {
    async fn stop(&self) -> anyhow::Result<()> {
        use anyhow::Context;
        self.stop().await.context("can't stop container")
    }
}
fn get_test_data_path(subpath: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("testdata");
    subpath.iter().for_each(|p| path.push(p));
    path
}

#[cfg(all(feature = "testcontainers", feature = "admin"))]
async fn async_tls_client(
    temp_dir: &tempfile::TempDir,
) -> Result<(cerbos::sdk::admin::CerbosAdminClient, impl Stoppable)> {
    use cerbos::sdk::{
        admin::CerbosAdminClient,
        container::{certs::CerbosTestTlsConfig, CerbosContainer},
        CerbosClientOptions, CerbosEndpoint,
    };
    use testcontainers::runners::AsyncRunner;

    let config_path = get_test_data_path(&["configs", "tcp_with_tls.yaml"]);

    let hostname = "localhost";
    let tls_config = CerbosTestTlsConfig::new(hostname, temp_dir)?;
    let container = CerbosContainer::default()
        .with_image_tag("0.45.1")
        .with_config_path(&config_path)
        .with_sqlite_in_memory_storage()
        .with_tls_config(&tls_config)
        .start()
        .await?;
    let host = container.get_host().await?;
    let port = container.get_host_port_ipv4(3593).await?;
    let client_conf = CerbosClientOptions::new(CerbosEndpoint::HostPort(host.to_string(), port))
        .with_admin_credentials(ADMIN_USERNAME, ADMIN_PASSWORD)
        .with_tls_ca_cert_pem(tls_config.get_ca_cert());
    Ok((CerbosAdminClient::new(client_conf).await?, container))
}
#[cfg(all(feature = "testcontainers", feature = "admin"))]
#[tokio::test]
pub async fn test_scratch() -> Result<()> {
    use cerbos::{
        genpb::cerbos::policy::v1::policy::PolicyType,
        sdk::{admin::model::PolicySet, deser::read_policy},
    };

    let policy_path = get_test_data_path(&["policies", "resource_policies", "policy_01.yaml"]);
    let file = std::fs::File::open(policy_path)?;
    let policy = read_policy(file)?;
    const RULE_ID: usize = 4;
    if let Some(PolicyType::ResourcePolicy(ref rp)) = policy.policy_type {
        println!("{:?}", rp.rules[RULE_ID]);
    } else {
        panic!("WTF");
    }
    let temp_dir = tempfile::TempDir::new()?;
    let (mut client, container) = async_tls_client(&temp_dir).await?;
    let mut policies = PolicySet::new();
    policies.add_policy(policy);
    client.add_or_update_policy(&policies).await?;
    let p = client
        .get_policy(vec!["resource.leave_request.v20210210".into()])
        .await?;
    if let Some(PolicyType::ResourcePolicy(ref rp)) = p[0].policy_type {
        println!("{:?}", rp.rules[RULE_ID]);
    }
    container.stop().await
}
#[cfg(all(feature = "testcontainers", feature = "admin"))]
#[tokio::test]
pub async fn test_cerbos_admin_client() -> Result<()> {
    let policies = HashMap::from([
        (
            "derived_roles.apatr_common_roles",
            "derived_roles/common_roles.yaml",
        ),
        ("derived_roles.alpha", "derived_roles/derived_roles_01.yaml"),
        ("derived_roles.beta", "derived_roles/derived_roles_02.yaml"),
        (
            "export_constants.bazqux",
            "export_constants/export_constants_01.yaml",
        ),
        (
            "export_variables.foobar",
            "export_variables/export_variables_01.yaml",
        ),
        (
            "principal.donald_duck.vdefault",
            "principal_policies/policy_02.yaml",
        ),
        (
            "principal.donald_duck.vdefault/acme",
            "principal_policies/policy_02_acme.yaml",
        ),
        (
            "principal.donald_duck.vdefault/acme.hr",
            "principal_policies/policy_02_acme.hr.yaml",
        ),
        (
            "resource.leave_request.v20210210",
            "resource_policies/policy_01.yaml",
        ),
        (
            "resource.leave_request.vdefault",
            "resource_policies/policy_05.yaml",
        ),
        (
            "resource.leave_request.vdefault/acme",
            "resource_policies/policy_05_acme.yaml",
        ),
        (
            "resource.leave_request.vdefault/acme.hr",
            "resource_policies/policy_05_acme.hr.yaml",
        ),
        (
            "resource.leave_request.vdefault/acme.hr.uk",
            "resource_policies/policy_05_acme.hr.uk.yaml",
        ),
    ]);

    let schemas = HashMap::from([
        ("principal.json", "_schemas/principal.json"),
        (
            "resources/leave_request.json",
            "_schemas/resources/leave_request.json",
        ),
        (
            "resources/purchase_order.json",
            "_schemas/resources/purchase_order.json",
        ),
    ]);
    let temp_dir = tempfile::TempDir::new()?;
    let (mut client, container) = async_tls_client(&temp_dir).await?;
    add_or_update_policies(&mut client, &policies).await?;
    list_policies(&mut client, &policies).await?;
    container.stop().await
}

async fn add_or_update_policies(
    client: &mut cerbos::sdk::admin::CerbosAdminClient,
    policies: &HashMap<&'static str, &'static str>,
) -> Result<()> {
    let mut ps = PolicySet::new();

    for p in policies.values() {
        let policy_path = get_test_data_path(&["policies", p]);
        ps.add_policy_from_file(policy_path)?;
    }
    client.add_or_update_policy(&ps).await
}
fn eq<A: Into<String>, B: Into<String>>(
    a: impl IntoIterator<Item = A>,
    b: impl IntoIterator<Item = B>,
) -> bool {
    let a: HashSet<_> = a.into_iter().map(|x| x.into()).collect();
    let b: HashSet<_> = b.into_iter().map(|x| x.into()).collect();
    a == b
}
async fn list_policies(
    client: &mut cerbos::sdk::admin::CerbosAdminClient,
    policies: &HashMap<&'static str, &'static str>,
) -> Result<()> {
    let ps = client.list_policies(None).await?;
    assert!(eq(ps, policies.keys().cloned()), "None filter");
    let fo = FilterOptions::new().with_scope_regexp("acme");
    let ps = client.list_policies(Some(fo.clone())).await?;
    assert!(eq(
        [
            "principal.donald_duck.vdefault/acme",
            "principal.donald_duck.vdefault/acme.hr",
            "resource.leave_request.vdefault/acme",
            "resource.leave_request.vdefault/acme.hr",
            "resource.leave_request.vdefault/acme.hr.uk",
        ],
        ps
    ));

    let fo = FilterOptions::new().with_name_regexp("leave_req");
    let ps = client.list_policies(Some(fo.clone())).await?;
    assert!(eq(
        [
            "resource.leave_request.v20210210",
            "resource.leave_request.vdefault",
            "resource.leave_request.vdefault/acme",
            "resource.leave_request.vdefault/acme.hr",
            "resource.leave_request.vdefault/acme.hr.uk",
        ],
        ps
    ));
    let fo = FilterOptions::new().with_version_regexp("\\d+");
    let ps = client.list_policies(Some(fo.clone())).await?;
    assert!(eq(["resource.leave_request.v20210210"], ps));

    let fo = FilterOptions::new()
        .with_name_regexp(".*")
        .with_scope_regexp(".*")
        .with_version_regexp("def");
    let ps = client.list_policies(Some(fo.clone())).await?;
    assert!(eq(
        [
            "principal.donald_duck.vdefault",
            "principal.donald_duck.vdefault/acme",
            "principal.donald_duck.vdefault/acme.hr",
            "resource.leave_request.vdefault",
            "resource.leave_request.vdefault/acme",
            "resource.leave_request.vdefault/acme.hr",
            "resource.leave_request.vdefault/acme.hr.uk"
        ],
        ps
    ));
    Ok(())
}

#[test]
pub fn test_google_protobuf_value_de() {
    use google::protobuf::{value::Kind, Value};
    let v: Value = serde_json::from_str("42").unwrap();

    assert!(matches!(v.kind, Some(Kind::NumberValue(42.0))));
}
