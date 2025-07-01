// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Deserializer};

use crate::genpb::cerbos::effect::v1::Effect;

pub(crate) fn deserialize_effect<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Effect::from_str_name(&s)
        .map(|e| e as i32)
        .ok_or_else(|| serde::de::Error::custom(format!("Unknown effect: {}", s)))
}
