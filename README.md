# Cerbos Rust SDK

Rust client library for [Cerbos](https://cerbos.dev): the open core, language-agnostic, scalable authorization solution that makes user permissions and authorization simple to implement and manage by writing context-aware access control policies for your application resources.

* [Cerbos website](https://cerbos.dev)
* [Cerbos documentation](https://docs.cerbos.dev)
* [Cerbos GitHub repository](https://github.com/cerbos/cerbos)
* [Cerbos Slack community](http://go.cerbos.io/slack)


## Usage

The client can be used either asynchronously or synchronously by instantiating `CerbosAsyncClient`
or `CerbosSyncClient` respectively.


```rust
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

    println!("Allowed={:?}", resp);

    Ok(())
}

```

