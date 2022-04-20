// Copyright 2021-2022 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
use cerbos::sdk::{
    attr::{Attr, StrVal},
    model::*,
    CerbosAsyncClient, CerbosClientOptions, CerbosEndpoint, Result,
};

async fn async_tls_client() -> Result<CerbosAsyncClient> {
    let client_conf = CerbosClientOptions::new(CerbosEndpoint::HostPort("localhost", 3593));
    CerbosAsyncClient::new(client_conf).await
}

async fn async_plaintext_client() -> Result<CerbosAsyncClient> {
    let client_conf =
        CerbosClientOptions::new(CerbosEndpoint::HostPort("localhost", 3593)).with_plaintext();
    CerbosAsyncClient::new(client_conf).await
}

#[tokio::test]
#[ignore]
async fn check_resources_tls() -> Result<()> {
    /*
    let docker = clients::Cli::default();
    let cerbos_container = docker.run(CerbosContainer::default().with_image_tag("dev"));
    let host = cerbos_container.get_bridge_ip_address().to_string();
    let port = cerbos_container.get_host_port(3593);
    */

    let client = async_tls_client().await?;
    do_check_resources(client).await
}

#[tokio::test]
async fn check_resources_plaintext() -> Result<()> {
    let client = async_plaintext_client().await?;
    do_check_resources(client).await
}

async fn do_check_resources(mut client: CerbosAsyncClient) -> Result<()> {
    let principal = Principal::new("alice", ["employee"])
        .with_policy_version("20210210")
        .with_attributes([
            Attr("department", StrVal("marketing")),
            Attr("geography", StrVal("GB")),
            Attr("team", StrVal("design")),
        ]);

    let resource = Resource::new("XX125", "leave_request")
        .with_policy_version("20210210")
        .with_attributes([
            Attr("department", StrVal("marketing")),
            Attr("geography", StrVal("GB")),
            Attr("team", StrVal("design")),
            Attr("owner", StrVal("alice")),
            Attr("id", StrVal("XX125")),
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
    assert!(allowed == true);

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
            Attr("department", StrVal("marketing")),
            Attr("geography", StrVal("GB")),
            Attr("team", StrVal("design")),
        ]);

    let resource = Resource::new("XX125", "leave_request")
        .with_policy_version("20210210")
        .with_attributes([
            Attr("department", StrVal("marketing")),
            Attr("geography", StrVal("GB")),
            Attr("team", StrVal("design")),
            Attr("owner", StrVal("alice")),
            Attr("id", StrVal("XX125")),
        ]);

    let allowed = client
        .is_allowed("view:public", principal, resource, None)
        .await?;
    assert!(allowed == true);

    Ok(())
}
