use cerbos_rs::sdk::model::Principal;
use cerbos_rs::sdk::model::Resource;
use cerbos_rs::sdk::model::ResourceList;
use cerbos_rs::sdk::CerbosAsyncClient;
use cerbos_rs::sdk::CerbosClientOptions;
use cerbos_rs::sdk::Result;
use prost_types::{value::Kind, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = CerbosClientOptions::new("https://localhost:3593");
    let mut client = CerbosAsyncClient::new(opt).await?;

    let principal = Principal::new("alice", vec!["employee"]).with_policy_version("20210210");
    let resource = Resource::new("XX125", "leave_request").with_policy_version("20210210");

    let resp = client
        .check_resources(
            principal,
            ResourceList::new().add(resource, vec!["view:public"]),
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

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pem = tokio::fs::read("tls.crt").await?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("cerbos.local");

    let channel = Channel::from_static("https://localhost:3593")
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = CerbosServiceClient::new(channel);

    let channel = Channel::from_static("https://localhost:3593")
        .tls_config(ClientTlsConfig::new())?
        .connect()
        .await?;

    let mut client = CerbosServiceClient::new(channel);
    //    let mut client = CerbosServiceClient::connect("http://localhost:3593").await?;
    let mut principal_attr: HashMap<String, Value> = HashMap::new();
    principal_attr.insert(
        "x".to_string(),
        Value {
            kind: Some(Kind::StringValue("y".to_string())),
        },
    );

    let mut resource_attr: HashMap<String, Value> = HashMap::new();
    resource_attr.insert(
        "x".to_string(),
        Value {
            kind: Some(Kind::StringValue("y".to_string())),
        },
    );

    let mut resource = Resource::default();
    resource.id = "test".to_string();
    resource.id = "test".to_string();
    resource.kind = "leave_request".to_string();
    resource.policy_version = "default".to_string();
    resource.attr = resource_attr;

    let mut principal = Principal::default();
    principal.id = "test".to_string();
    principal.policy_version = "default".to_string();
    principal.roles = vec!["admin".to_string()];
    principal.attr = principal_attr;

    let mut req = CheckResourceBatchRequest::default();
    req.request_id = "test".to_string();
    req.principal = Some(principal);
    req.resources = vec![BatchEntry {
        actions: vec!["test".to_string()],
        resource: Some(resource),
    }];

    let response = client
        .check_resource_batch(tonic::Request::new(req))
        .await?;

    println!("Response={:?}", response);
    Ok(())
}
*/
