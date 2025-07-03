// Copyright 2021-2022 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
use cerbos::sdk::attr::attr;
use cerbos::sdk::model::{Principal, Resource};
use cerbos::sdk::{CerbosAsyncClient, CerbosClientOptions, CerbosEndpoint, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let opt = CerbosClientOptions::new(CerbosEndpoint::HostPort("localhost", 3593));
    let mut client = CerbosAsyncClient::new(opt).await?;

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
            attr("approved", true),
            attr("id", "XX125"),
        ]);

    let resp = client
        .is_allowed("view:public", principal, resource, None)
        .await?;

    println!("Allowed={resp:?}");

    Ok(())
}
