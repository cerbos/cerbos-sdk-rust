fn main() -> Result<(), std::io::Error> {
    let mut builder = tonic_build::configure()
        .out_dir("src/genpb")
        .build_server(false);

    if cfg!(feature = "admin") {
        let types = [
            "Condition",
            "ExportVariables",
            "Variables",
            "ExportConstants",
            "Constants",
            "DerivedRoles",
            "RoleDef",
            "RolePolicy",
            "RoleRule",
            "PrincipalPolicy",
            "PrincipalRule",
            "PrincipalRule.Action",
            "ResourcePolicy",
            "ResourceRule",
            "ResourceRule.Action",
            "Policy",
            "Metadata",
            "Match",
            "Match.ExprList",
            "Output",
            "Output.When",
            "Schemas",
            "Schemas.Schema",
            "Schemas.IgnoreWhen",
            // "TestFixture",
            // "TestFixture.Principals",
            // "TestFixture.Resources",
            // "TestFixture.AuxData",
            // "TestFixtureGroup",
            // "TestFixtureGroup.Principals",
            // "TestFixtureGroup.Resources",
            // "TestOptions",
            "Policy.policy_type",
            "RolePolicy.policy_type",
            "Match.op",
            "Condition.condition",
            "SourceAttributes",
        ];
        for t in types {
            builder = builder.type_attribute(
                format!("cerbos.policy.v1.{}", t),
                "#[derive(serde::Deserialize)]",
            );
        }
        for t in [
            "Value",
            "Value.kind",
            "ListValue",
            "Struct",
            "Timestamp",
            "UInt64Value",
        ] {
            builder = builder.type_attribute(
                format!("google.protobuf.{}", t),
                "#[derive(serde::Deserialize)]",
            );
        }
    }

    builder.compile_well_known_types(true).compile_protos(
        &[
            "proto/defs/cerbos/policy/v1/policy.proto",
            "proto/defs/cerbos/svc/v1/svc.proto",
            "proto/defs/cerbos/cloud/store/v1/store.proto",
            "proto/defs/cerbos/cloud/apikey/v1/apikey.proto",
            "proto/defs/google/rpc/status.proto",
            "proto/defs/google/protobuf/timestamp.proto",
            "proto/defs/google/protobuf/struct.proto",
        ],
        &["proto/defs/"],
    )
}
