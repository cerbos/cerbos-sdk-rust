// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
use std::path::PathBuf;

use anyhow::Result;

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
    path.push("resources");
    path.push("store");
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

    let policies_dir = get_test_data_path(&["policies"]);
    let config_path = get_test_data_path(&["configs", "tcp_with_tls.yaml"]);

    let hostname = "localhost";
    let tls_config = CerbosTestTlsConfig::new(hostname, temp_dir)?;
    let container = CerbosContainer::default()
        .with_image_tag("0.44.0")
        .with_config_path(&config_path)
        .with_sqlite_in_memory_storage()
        .with_tls_config(&tls_config)
        .with_extra_volume_mounts(vec![(policies_dir.to_str().unwrap(), "/policies")])
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
pub async fn test_save_read_policy() -> Result<()> {
    use cerbos::{
        genpb::cerbos::policy::v1::policy::PolicyType,
        sdk::{admin::model::PolicySet, deser::read_policy},
    };

    let policy_path = get_test_data_path(&["resource_policies", "policy_01.yaml"]);
    let file = std::fs::File::open(policy_path)?;
    let policy = read_policy(file)?;
    if let Some(PolicyType::ResourcePolicy(ref rp)) = policy.policy_type {
        println!("{:?}", rp.rules[0]);
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
    println!("{:#?}", p);
    container.stop().await
}
#[tokio::test]
pub async fn test_reading_policy() -> Result<()> {
    use cerbos::{genpb::cerbos::policy::v1::policy::PolicyType, sdk::deser::read_policy};

    let policy_path = get_test_data_path(&["resource_policies", "policy_01.yaml"]);
    let file = std::fs::File::open(policy_path)?;
    let policy = read_policy(file)?;
    // let mut buf = vec![];
    // _ = file.read_to_end(&mut buf)?;
    // let yaml_value: YamlValue = serde_yml::from_slice(&buf)?;
    // let resource_policy_value = yaml_value.get("resourcePolicy").unwrap().clone();
    // let policy: Policy = serde_yml::from_value(yaml_value)?;
    // let rp: ResourcePolicy = serde_yml::from_value(resource_policy_value)?;
    // let p: Policy = serde_yml::from_slice(&buf)?;
    println!("Policy vars: {:?}", policy.variables);
    if let Some(PolicyType::ResourcePolicy(rp)) = policy.policy_type {
        println!("{}", rp.resource);

        let rs = rp.schemas.unwrap().resource_schema.unwrap().r#ref;
        let schema_file = std::fs::File::open(get_test_data_path(&["_schemas", &rs]))?;
        // let schema = read_schema(schema_file)?;
        // println!("{}", schema.definition);
    }
    // if let p.po
    // if let Some(PolicyType::ResourcePolicy(ref rp)) = p.policy_type {
    //     println!("{} {}", rp.resource, rp.version);
    // } else {
    //     println!("{:?}", p.policy_type)
    // }
    Ok(())
}
