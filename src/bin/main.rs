use cerbos_rs::sdk::attr::{Attr, StrVal};
use cerbos_rs::sdk::model::Principal;
use cerbos_rs::sdk::model::Resource;
use cerbos_rs::sdk::model::ResourceAction;
use cerbos_rs::sdk::model::ResourceList;
use cerbos_rs::sdk::CerbosClientOptions;
use cerbos_rs::sdk::Result;
use cerbos_rs::sdk::{CerbosAsyncClient, CerbosEndpoint};

#[tokio::main]
async fn main() -> Result<()> {
    let opt = CerbosClientOptions::new(CerbosEndpoint::HostPort("localhost", 3593));
    let mut client = CerbosAsyncClient::new(opt).await?;

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

    println!("Response={:?}", resp);
    println!(
        "Allowed={:?}",
        resp.find("XX125").map(|r| r.is_allowed("view:public"))
    );
    Ok(())
}
