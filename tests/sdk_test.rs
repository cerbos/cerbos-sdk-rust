// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use cerbos::{
    genpb::google::protobuf::{value, ListValue, Struct, Value},
    sdk::{attr::attr, model::*, CerbosAsyncClient, CerbosClientOptions, CerbosEndpoint, Result},
};

#[cfg(not(feature = "testcontainers"))]
async fn async_plaintext_client() -> Result<CerbosAsyncClient> {
    let client_conf =
        CerbosClientOptions::new(CerbosEndpoint::HostPort("localhost", 3593)).with_plaintext();
    CerbosAsyncClient::new(client_conf).await
}

#[cfg(feature = "testcontainers")]
trait Stoppable {
    async fn stop(&self) -> anyhow::Result<()>;
}

#[cfg(feature = "testcontainers")]
impl<T: testcontainers::Image> Stoppable for testcontainers::ContainerAsync<T> {
    async fn stop(&self) -> anyhow::Result<()> {
        use anyhow::Context;
        self.stop_with_timeout(None)
            .await
            .context("can't stop container")
    }
}
#[cfg(feature = "testcontainers")]
async fn async_tls_client(
    temp_dir: &tempfile::TempDir,
) -> Result<(CerbosAsyncClient, impl Stoppable)> {
    use cerbos::sdk::container::{certs::CerbosTestTlsConfig, CerbosContainer};
    use testcontainers::runners::AsyncRunner;

    let mut store_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    store_dir.push("resources");
    store_dir.push("store");

    let hostname = "localhost";
    let config = CerbosTestTlsConfig::new(hostname, temp_dir)?;
    let container = CerbosContainer::default()
        .with_image_tag("0.44.0")
        .with_extra_volume_mounts(vec![(store_dir.to_str().unwrap(), "/policies")])
        .with_tls_config(&config)
        .start()
        .await?;
    let host = container.get_host().await?;
    let port = container.get_host_port_ipv4(3593).await?;
    let client_conf = CerbosClientOptions::new(CerbosEndpoint::HostPort(host.to_string(), port))
        .with_tls_ca_cert_pem(config.get_ca_cert());
    Ok((CerbosAsyncClient::new(client_conf).await?, container))
}

#[cfg(feature = "testcontainers")]
#[tokio::test]
async fn check_resources_tls() -> Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let (client, container) = async_tls_client(&temp_dir).await?;
    do_check_resources(client).await?;
    container.stop().await
}

#[tokio::test]
#[cfg(not(feature = "testcontainers"))]
async fn check_resources_plaintext() -> Result<()> {
    let client = async_plaintext_client().await?;
    do_check_resources(client).await
}

#[cfg(feature = "testcontainers")]
#[tokio::test]
async fn check_resources_tls_with_output() -> Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let (client, contiainer) = async_tls_client(&temp_dir).await?;
    do_check_resources_with_output(client).await?;
    contiainer.stop().await
}

#[cfg(not(feature = "testcontainers"))]
#[tokio::test]
async fn check_resources_plaintext_with_output() -> Result<()> {
    let client = async_plaintext_client().await?;
    do_check_resources_with_output(client).await
}

fn string_value(s: impl Into<String>) -> Value {
    Value {
        kind: Some(value::Kind::StringValue(s.into())),
    }
}

fn bool_value(b: bool) -> Value {
    Value {
        kind: Some(value::Kind::BoolValue(b)),
    }
}

fn number_value(f: f64) -> Value {
    Value {
        kind: Some(value::Kind::NumberValue(f)),
    }
}

fn list_value(values: Vec<Value>) -> Value {
    Value {
        kind: Some(value::Kind::ListValue(ListValue { values })),
    }
}

fn struct_value(fields: Vec<(String, Value)>) -> Value {
    Value {
        kind: Some(value::Kind::StructValue(Struct {
            fields: fields.into_iter().collect(),
        })),
    }
}

async fn do_check_resources(mut client: CerbosAsyncClient) -> Result<()> {
    let principal = Principal::new("alice", ["employee"])
        .with_policy_version("20210210")
        .with_attributes([
            attr("department", "marketing"),
            attr("geography", "GB"),
            attr("team", "design"),
        ]);

    let resource = Resource::new("XX125", "leave_request")
        .with_policy_version("20210210")
        .with_attributes([
            attr("department", "marketing"),
            attr("geography", "GB"),
            attr("team", "design"),
            attr("owner", "alice"),
            attr("id", "XX125"),
        ]);

    let resp = client
        .check_resources(
            principal,
            ResourceList::new_from([ResourceAction(resource, ["view:public"])]),
            None,
        )
        .await?;

    let allowed = resp
        .find("XX125")
        .map(|x| x.is_allowed("view:public"))
        .unwrap();
    assert!(allowed);

    Ok(())
}

async fn do_check_resources_with_output(mut client: CerbosAsyncClient) -> Result<()> {
    let principal = Principal::new("donald_duck", ["employee"]).with_policy_version("20210210");

    let resource = Resource::new("XX125", "leave_request")
        .with_policy_version("20210210")
        .with_attributes([attr("id", "XX125")]);

    let resp = client
        .check_resources(
            principal,
            ResourceList::new_from([ResourceAction(resource, ["view:public"])]),
            None,
        )
        .await?;

    let xx125_or_none = resp.find("XX125");
    assert!(xx125_or_none.is_some());

    let xx125 = xx125_or_none.unwrap();

    let resource_output = Some(Value {
        kind: Some(value::Kind::StructValue(Struct {
            fields: [
                (
                    "formatted_string".to_string(),
                    string_value("id:donald_duck"),
                ),
                ("keys".to_string(), string_value("XX125")),
                ("pID".to_string(), string_value("donald_duck")),
                ("some_bool".to_string(), bool_value(true)),
                (
                    "some_list".to_string(),
                    list_value(vec![string_value("foo"), string_value("bar")]),
                ),
                (
                    "something_nested".to_string(),
                    struct_value(vec![
                        ("nested_str".to_string(), string_value("foo")),
                        ("nested_bool".to_string(), bool_value(false)),
                        (
                            "nested_list".to_string(),
                            list_value(vec![string_value("nest_foo"), number_value(1.01)]),
                        ),
                        (
                            "nested_formatted_string".to_string(),
                            string_value("id:donald_duck"),
                        ),
                    ]),
                ),
            ]
            .into_iter()
            .collect(),
        })),
    });
    assert_eq!(
        xx125.output("resource.leave_request.v20210210#public-view"),
        resource_output.as_ref()
    );

    assert!(xx125.output("nonexistent key").is_none());

    let allowed = resp
        .find("XX125")
        .map(|x| x.is_allowed("view:public"))
        .unwrap();
    assert!(allowed);

    Ok(())
}

#[cfg(feature = "testcontainers")]
#[tokio::test]
async fn is_allowed_tls() -> Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let (client, container) = async_tls_client(&temp_dir).await?;
    do_is_allowed(client).await?;
    container.stop().await
}

#[cfg(not(feature = "testcontainers"))]
#[tokio::test]
async fn is_allowed_plaintext() -> Result<()> {
    let client = async_plaintext_client().await?;
    do_is_allowed(client).await
}

async fn do_is_allowed(mut client: CerbosAsyncClient) -> Result<()> {
    let principal = Principal::new("alice", ["employee"])
        .with_policy_version("20210210")
        .with_attributes([
            attr("department", "marketing"),
            attr("geography", "GB"),
            attr("team", "design"),
        ]);

    let resource = Resource::new("XX125", "leave_request")
        .with_policy_version("20210210")
        .with_attributes([
            attr("department", "marketing"),
            attr("geography", "GB"),
            attr("team", "design"),
            attr("owner", "alice"),
            attr("id", "XX125"),
        ]);

    let allowed = client
        .is_allowed("view:public", principal, resource, None)
        .await?;
    assert!(allowed);

    Ok(())
}

#[cfg(feature = "testcontainers")]
#[tokio::test]
async fn plan_resources_tls() -> Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let (client, container) = async_tls_client(&temp_dir).await?;
    do_plan_resources(client).await?;
    container.stop().await
}

#[cfg(not(feature = "testcontainers"))]
#[tokio::test]
async fn plan_resources_plaintext() -> Result<()> {
    let client = async_plaintext_client().await?;
    do_plan_resources(client).await
}

async fn do_plan_resources(mut client: CerbosAsyncClient) -> Result<()> {
    let principal = Principal::new("maggie", ["manager", "employee"])
        .with_policy_version("20210210")
        .with_attributes([
            attr("department", "marketing"),
            attr("geography", "GB"),
            attr("managed_geographies", "GB"),
            attr("team", "design"),
        ]);

    let resource = ResourceKind::new("leave_request").with_policy_version("20210210");

    let response = client
        .plan_resources("approve", principal.clone(), resource.clone(), None)
        .await?;
    assert!(matches!(
        response.filter(),
        PlanResourcesFilter::Conditional(..)
    ));

    let response = client
        .plan_resources_for_actions(["approve", "view:public"], principal, resource, None)
        .await?;

    assert!(matches!(
        response.filter(),
        PlanResourcesFilter::Conditional(..)
    ));

    Ok(())
}
