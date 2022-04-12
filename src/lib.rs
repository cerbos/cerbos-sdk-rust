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
    }
}

