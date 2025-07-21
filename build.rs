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
        "Policy.policy_type",
        "RolePolicy.policy_type",
        "Match.op",
        "Condition.condition",
        "SourceAttributes",
    ];
    for t in types {
        // builder = add_serde_annotations(builder, "cerbos.policy.v1.", t);
    }
    // for t in ["UInt64Value"] {
    //     builder = add_serde_annotations(builder, "google.protobuf.", t);
    // }
    let deser_effect_attr = "#[cfg_attr(feature = \"serde\", serde(deserialize_with = \"crate::sdk::deser::deserialize_effect\"))]";
    let flatten_attr = "#[cfg_attr(feature = \"serde\", serde(flatten))]";
    let if_struct_serde_default = "#[if_struct_macro::serde_default]";
    let deser_rename_attr =  "#[cfg_attr(feature = \"serde\", derive(serde::Deserialize), serde(rename_all = \"camelCase\"))]";
    builder = builder
        .type_attribute(".cerbos.policy.v1", if_struct_serde_default)
        .enum_attribute(".cerbos.policy.v1", deser_rename_attr)
        .type_attribute(".cerbos.engine.v1", deser_rename_attr)
        .type_attribute(".cerbos.effect.v1", deser_rename_attr)
        .type_attribute(".cerbos.schema.v1", deser_rename_attr)
        .type_attribute(".google.api.expr.v1alpha1", deser_rename_attr)
        .type_attribute("google.protobuf.Empty", deser_rename_attr)
        .type_attribute("google.protobuf.UInt64Value", deser_rename_attr)
        .type_attribute("google.protobuf.Timestamp", deser_rename_attr)
        .type_attribute("google.protobuf.Duration", deser_rename_attr)
        .field_attribute("ResourceRule.effect", deser_effect_attr)
        .field_attribute("PrincipalRule.Action.effect", deser_effect_attr)
        .field_attribute("Match.op", flatten_attr)
        .field_attribute("Condition.condition", flatten_attr);

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
    let mut b = builder;
    if is_enum(t) {
        b = b.type_attribute(
            format!("{prefix}{t}"),
            "#[cfg_attr(feature = \"serde\", derive(serde::Deserialize), serde(rename_all = \"camelCase\"))]",
        );
    }
    b
}

fn is_enum(s: &str) -> bool {
    s.ends_with(".policy_type")
        || s.ends_with(".op")
        || s == "Condition.condition"
        || s == "Value.kind"
}
