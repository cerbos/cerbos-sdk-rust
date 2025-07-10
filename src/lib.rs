// Copyright 2021-2022 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0
//
//! Client library for communicating with a [Cerbos](https://cerbos.dev) policy decision point (PDP). Cerbos is the open core, language-agnostic, scalable
//! authorization solution that makes user permissions and authorization simple to implement
//! and manage by writing context-aware access control policies for your application resources.
//! See https://docs.cerbos.dev for more information about Cerbos.
//!
//! ## Example
//!
//! ```rust,no_run
//! use cerbos::sdk::attr::attr;
//! use cerbos::sdk::model::{Principal, Resource};
//! use cerbos::sdk::{CerbosAsyncClient, CerbosClientOptions, CerbosEndpoint, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let opt = CerbosClientOptions::new(CerbosEndpoint::HostPort("localhost", 3593));
//!     let mut client = CerbosAsyncClient::new(opt).await?;
//!
//!     let principal = Principal::new("alice", ["employee"])
//!         .with_policy_version("20210210")
//!         .with_attributes([
//!             attr("department", "marketing"),
//!             attr("geography", "GB"),
//!             attr("team", "design"),
//!         ]);
//!
//!     let resource = Resource::new("XX125", "leave_request")
//!         .with_policy_version("20210210")
//!         .with_attributes([
//!             attr("department", "marketing"),
//!             attr("geography", "GB"),
//!             attr("team", "design"),
//!             attr("owner", "alice"),
//!             attr("approved", true),
//!             attr("id", "XX125"),
//!         ]);
//!
//!     let resp = client
//!         .is_allowed("view:public", principal, resource, None)
//!         .await?;
//!
//!     println!("Allowed={:?}", resp);
//!
//!     Ok(())
//! }
//!
//! ```

#[allow(clippy::large_enum_variant, clippy::doc_overindented_list_items)]
#[path = "genpb"]
pub mod genpb {
    #[path = ""]
    pub mod cerbos {
        #[path = ""]
        pub mod audit {
            #[path = "cerbos.audit.v1.rs"]
            pub mod v1;
        }

        #[path = ""]
        pub mod effect {
            #[path = "cerbos.effect.v1.rs"]
            pub mod v1;
        }

        #[path = ""]
        pub mod engine {
            #[path = "cerbos.engine.v1.rs"]
            pub mod v1;
        }

        #[path = ""]
        pub mod policy {
            #[path = "cerbos.policy.v1.rs"]
            pub mod v1;
        }

        #[path = ""]
        pub mod schema {
            #[path = "cerbos.schema.v1.rs"]
            pub mod v1;
        }

        #[path = ""]
        pub mod request {
            #[path = "cerbos.request.v1.rs"]
            pub mod v1;
        }

        #[path = ""]
        pub mod response {
            #[path = "cerbos.response.v1.rs"]
            pub mod v1;
        }

        #[path = ""]
        pub mod svc {
            #[path = "cerbos.svc.v1.rs"]
            pub mod v1;
        }
        #[path = ""]
        pub mod cloud {
            #[path = ""]
            pub mod store {
                #[path = "cerbos.cloud.store.v1.rs"]
                pub mod v1;
            }
            #[path = ""]
            pub mod apikey {
                #[path = "cerbos.cloud.apikey.v1.rs"]
                pub mod v1;
            }
        }
    }

    #[path = ""]
    pub mod google {
        #[path = ""]
        pub mod api {
            #[path = ""]
            pub mod expr {
                #[path = "google.api.expr.v1alpha1.rs"]
                pub mod v1alpha1;
            }
        }
        #[path = "google.protobuf.rs"]
        pub mod protobuf;
        #[path = "google.rpc.rs"]
        pub mod rpc;
    }
}

pub mod sdk;
