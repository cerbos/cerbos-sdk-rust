// Copyright 2021-2022 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::time::Duration;

use cerbos::sdk::{
    attr::attr, model::*, CerbosAsyncClient, CerbosClientOptions, CerbosEndpoint, Result,
};
use prost::bytes::BufMut;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct, Value};

#[cfg(feature = "testcontainers")]
use testcontainers::{
    core::{IntoContainerPort, Mount, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};

#[cfg(not(feature = "testcontainers"))]
async fn async_tls_client() -> Result<CerbosAsyncClient> {
    let client_conf = CerbosClientOptions::new(CerbosEndpoint::HostPort("localhost", 3593));
    CerbosAsyncClient::new(client_conf).await
}

#[cfg(not(feature = "testcontainers"))]
async fn async_plaintext_client() -> Result<CerbosAsyncClient> {
    let client_conf =
        CerbosClientOptions::new(CerbosEndpoint::HostPort("localhost", 3593)).with_plaintext();
    CerbosAsyncClient::new(client_conf).await
}

#[cfg(feature = "testcontainers")]
async fn async_plaintext_client() -> Result<CerbosAsyncClient> {
    let mut store_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    store_dir.push("resources");
    store_dir.push("store");

    let http_port = 3592;
    let grpc_port = 3593;
    let container = GenericImage::new("ghcr.io/cerbos/cerbos", "0.43.0")
        .with_wait_for(WaitFor::message_on_stdout("Starting HTTP server"))
        .with_exposed_port(http_port.tcp())
        .with_exposed_port(grpc_port.tcp())
        .with_env_var("CERBOS_NO_TELEMETRY", "1")
        .with_network("bridge")
        .with_mount(Mount::bind_mount(store_dir.to_str().unwrap(), "/policies"))
        .start()
        .await?;
    let host = container.get_host().await?;
    // let host = container.get_bridge_ip_address().await?;
    let gport = container.get_host_port_ipv4(grpc_port).await?;
    let hport = container.get_host_port_ipv4(http_port).await?;
    println!("host: {} gRPC port: {} HTTP port: {}", host, gport, hport);
    tokio::time::sleep(Duration::from_secs(5)).await;
    let output = container.stdout_to_vec().await?;
    println!("{}", String::from_utf8(output)?);
    let client_conf =
        CerbosClientOptions::new(CerbosEndpoint::HostPort(host.to_string(), grpc_port))
            .with_plaintext();
    CerbosAsyncClient::new(client_conf).await
}

#[tokio::test]
#[ignore]
async fn check_resources_tls() -> Result<()> {
    let client = async_tls_client().await?;
    do_check_resources(client).await
}

#[tokio::test]
async fn check_resources_plaintext() -> Result<()> {
    let client = async_plaintext_client().await?;
    do_check_resources(client).await
}

#[tokio::test]
#[ignore]
async fn check_resources_tls_with_output() -> Result<()> {
    let client = async_tls_client().await?;
    do_check_resources_with_output(client).await
}

#[tokio::test]
async fn check_resources_plaintext_with_output() -> Result<()> {
    let client = async_plaintext_client().await?;
    do_check_resources_with_output(client).await
}

fn string_value(s: impl Into<String>) -> Value {
    Value {
        kind: Some(Kind::StringValue(s.into())),
    }
}

fn bool_value(b: bool) -> Value {
    Value {
        kind: Some(Kind::BoolValue(b)),
    }
}

fn number_value(f: f64) -> Value {
    Value {
        kind: Some(Kind::NumberValue(f)),
    }
}

fn list_value(values: Vec<Value>) -> Value {
    Value {
        kind: Some(Kind::ListValue(ListValue { values })),
    }
}

fn struct_value(fields: Vec<(String, Value)>) -> Value {
    Value {
        kind: Some(Kind::StructValue(Struct {
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
    assert!(!xx125_or_none.is_none());

    let xx125 = xx125_or_none.unwrap();

    let resource_output = Some(Value {
        kind: Some(Kind::StructValue(Struct {
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

#[tokio::test]
#[ignore]
async fn is_allowed_tls() -> Result<()> {
    let client = async_tls_client().await?;
    do_is_allowed(client).await
}

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

#[tokio::test]
#[ignore]
async fn plan_resources_tls() -> Result<()> {
    let client = async_tls_client().await?;
    do_plan_resources(client).await
}

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

    // let response = client
    //     .plan_resources_for_actions(["approve", "view:public"], principal, resource, None)
    //     .await?;

    assert!(matches!(
        response.filter(),
        PlanResourcesFilter::Conditional(..)
    ));

    Ok(())
}
