fn main() -> Result<(), std::io::Error> {
    let mut builder = tonic_prost_build::configure()
        .out_dir("src/genpb")
        .build_server(false);

    let de_effect = "#[cfg_attr(feature = \"serde\", serde(deserialize_with = \"crate::sdk::deser::deserialize_effect\"))]";
    let flatten = "#[cfg_attr(feature = \"serde\", serde(flatten))]";
    let de =  "#[cfg_attr(feature = \"serde\", derive(serde::Deserialize), serde(rename_all = \"camelCase\"))]";
    builder = builder
        .type_attribute(".cerbos.policy.v1", "#[if_struct_macro::serde_default]")
        .enum_attribute(".cerbos.policy.v1", de)
        .type_attribute(".cerbos.engine.v1", de)
        .type_attribute(".cerbos.effect.v1", de)
        .type_attribute(".cerbos.schema.v1", de)
        .type_attribute(".google.api.expr.v1alpha1", de)
        .type_attribute("google.protobuf.Empty", de)
        .type_attribute("google.protobuf.UInt64Value", de)
        .type_attribute("google.protobuf.Timestamp", de)
        .type_attribute("google.protobuf.Duration", de)
        .field_attribute("ResourceRule.effect", de_effect)
        .field_attribute("PrincipalRule.Action.effect", de_effect)
        .field_attribute("Match.op", flatten)
        .field_attribute("Condition.condition", flatten);

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
