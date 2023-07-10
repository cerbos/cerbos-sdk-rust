#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Effect {
    Unspecified = 0,
    Allow = 1,
    Deny = 2,
    NoMatch = 3,
}
impl Effect {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Effect::Unspecified => "EFFECT_UNSPECIFIED",
            Effect::Allow => "EFFECT_ALLOW",
            Effect::Deny => "EFFECT_DENY",
            Effect::NoMatch => "EFFECT_NO_MATCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EFFECT_UNSPECIFIED" => Some(Self::Unspecified),
            "EFFECT_ALLOW" => Some(Self::Allow),
            "EFFECT_DENY" => Some(Self::Deny),
            "EFFECT_NO_MATCH" => Some(Self::NoMatch),
            _ => None,
        }
    }
}
