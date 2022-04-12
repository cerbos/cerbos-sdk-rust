#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Effect {
    Unspecified = 0,
    Allow = 1,
    Deny = 2,
    NoMatch = 3,
}
