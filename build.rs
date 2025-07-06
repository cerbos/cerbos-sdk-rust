fn main() -> Result<(), std::io::Error> {
    let mut builder = tonic_build::configure()
        .out_dir("src/genpb")
        .build_server(false);

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
        builder = add_serde_annotations(builder, "cerbos.policy.v1.", t);
    }
    for t in [
        "Value",
        "Value.kind",
        "ListValue",
        "Struct",
        "Timestamp",
        "UInt64Value",
    ] {
        builder = add_serde_annotations(builder, "google.protobuf.", t);
    }
    let deser_effect_attr = "#[cfg_attr(feature = \"serde\", serde(deserialize_with = \"crate::sdk::deser::deserialize_effect\"))]";
    builder = builder
        .field_attribute("ResourceRule.effect", deser_effect_attr)
        .field_attribute("PrincipalRule.Action.effect", deser_effect_attr);

    builder = add_serde_annotations(builder, "cerbos.schema.v1.", "Schema");

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

fn add_serde_annotations(
    builder: tonic_build::Builder,
    prefix: &'static str,
    t: &'static str,
) -> tonic_build::Builder {
    let mut b = builder.type_attribute(
            format!("{}{}", prefix, t),
            "#[cfg_attr(feature = \"serde\", derive(serde::Deserialize), serde(rename_all = \"camelCase\"))]",
        );
    if !is_enum(t) {
        b = b.type_attribute(
            format!("{}{}", prefix, t),
            "#[cfg_attr(feature = \"serde\", serde(default))]",
        );
    }
    b
}

fn is_enum(s: &str) -> bool {
    s.contains(".")
}
