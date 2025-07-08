// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{bail, Context};
use serde::{Deserialize, Deserializer};

use crate::genpb::cerbos::{
    effect::v1::Effect,
    policy::v1::{
        policy::PolicyType, DerivedRoles, ExportConstants, ExportVariables, Policy,
        PrincipalPolicy, ResourcePolicy, RolePolicy,
    },
    schema::v1::Schema,
};
use std::io::{BufRead, BufReader, Read};

pub mod value;

pub(crate) fn deserialize_effect<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Effect::from_str_name(&s)
        .map(|e| e as i32)
        .ok_or_else(|| serde::de::Error::custom(format!("Unknown effect: {}", s)))
}

const BUF_SIZE: usize = 1024 * 4; // 4KiB
const MAX_FILE_SIZE: usize = 1024 * 1024 * 4; // 4MiB
static JSON_START: &str = "{";

trait PolicyDeser {
    type Value: Clone;
    fn from_value<T: serde::de::DeserializeOwned>(&self, value: Self::Value) -> anyhow::Result<T>;
    fn from_self<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T>;
    fn get(&self, name: &str) -> Option<&Self::Value>;
}

struct JsonPolicyDeser(serde_json::Value);
impl JsonPolicyDeser {
    fn new(buf: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Self(
            serde_json::from_slice(&buf).with_context(|| "fail to deser from slice")?,
        ))
    }
}
impl PolicyDeser for JsonPolicyDeser {
    type Value = serde_json::Value;
    fn from_value<T: serde::de::DeserializeOwned>(&self, value: Self::Value) -> anyhow::Result<T> {
        serde_json::from_value(value).with_context(|| "fail to deserialize")
    }

    fn get(&self, name: &str) -> Option<&Self::Value> {
        self.0.get(name)
    }

    fn from_self<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T> {
        serde_json::from_value(self.0).with_context(|| "fail to deserialize")
    }
}

struct YamlPolicyDeser(serde_yml::Value);
impl YamlPolicyDeser {
    fn new(buf: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Self(
            serde_yml::from_slice(&buf).with_context(|| "fail to deser from slice")?,
        ))
    }
}
impl PolicyDeser for YamlPolicyDeser {
    type Value = serde_yml::Value;

    fn from_value<T: serde::de::DeserializeOwned>(&self, value: Self::Value) -> anyhow::Result<T> {
        serde_yml::from_value(value).with_context(|| "fail to deserialize")
    }

    fn get(&self, name: &str) -> Option<&Self::Value> {
        self.0.get(name)
    }

    fn from_self<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T> {
        serde_yml::from_value(self.0).with_context(|| "fail to deserialize")
    }
}
enum Decoder {
    Json(JsonPolicyDeser),
    Yaml(YamlPolicyDeser),
}
fn make_decoder(src: impl Read) -> anyhow::Result<Decoder> {
    let mut buf = BufReader::with_capacity(BUF_SIZE, src);

    // Peek at the buffer to determine format
    let prelude = buf.fill_buf().map(String::from_utf8_lossy)?;
    let trimmed = prelude.trim_start();
    let result = if trimmed.starts_with(JSON_START) {
        let mut h = buf.take(MAX_FILE_SIZE as u64);
        let mut data = Vec::new();
        h.read(&mut data)?;
        Decoder::Json(JsonPolicyDeser::new(data)?)
    } else {
        let h = buf.take(MAX_FILE_SIZE as u64);
        let data: Vec<u8> = parse_yaml(h)?;
        Decoder::Yaml(YamlPolicyDeser::new(data)?)
    };
    Ok(result)
}
fn make_policy(s: impl PolicyDeser) -> anyhow::Result<Policy> {
    let pt = if let Some(v) = s.get("resourcePolicy") {
        let p: ResourcePolicy = s.from_value(v.clone())?;
        Some(PolicyType::ResourcePolicy(p))
    } else if let Some(v) = s.get("principalPolicy") {
        let p: PrincipalPolicy = s.from_value(v.clone())?;
        Some(PolicyType::PrincipalPolicy(p))
    } else if let Some(v) = s.get("derivedRoles") {
        let p: DerivedRoles = s.from_value(v.clone())?;
        Some(PolicyType::DerivedRoles(p))
    } else if let Some(v) = s.get("exportVariables") {
        let p: ExportVariables = s.from_value(v.clone())?;
        Some(PolicyType::ExportVariables(p))
    } else if let Some(v) = s.get("exportConstants") {
        let p: ExportConstants = s.from_value(v.clone())?;
        Some(PolicyType::ExportConstants(p))
    } else if let Some(v) = s.get("rolePolicy") {
        let p: RolePolicy = s.from_value(v.clone())?;
        Some(PolicyType::RolePolicy(p))
    } else {
        None
    };
    let mut policy: Policy = s.from_self()?;
    policy.policy_type = pt;
    Ok(policy)
}

pub fn read_policy(src: impl Read) -> anyhow::Result<Policy> {
    match make_decoder(src)? {
        Decoder::Json(j) => make_policy(j),
        Decoder::Yaml(y) => make_policy(y),
    }
}
fn parse_yaml(reader: impl BufRead) -> anyhow::Result<Vec<u8>> {
    const YAML_SEP: &str = "---";
    const YAML_COMMENT: &'static str = "#";
    const NEWLINE: u8 = b'\n';

    let mut buf = Vec::new();
    let mut num_docs = 0;
    let mut seen_content = false;

    for line_result in reader.lines() {
        let line = line_result.with_context(|| format!("failed to read from source"))?;
        let trimmed_line = line.trim_start();

        if trimmed_line.starts_with(YAML_COMMENT) {
            continue;
        }

        if !seen_content && trimmed_line.is_empty() {
            continue;
        }
        seen_content = true;

        if line.starts_with(YAML_SEP) {
            num_docs += 1;
            if num_docs > 1 || (num_docs == 1 && !buf.is_empty()) {
                bail!("multiple YAML docs")
            }
        }

        buf.extend_from_slice(line.as_bytes());
        buf.push(NEWLINE);
    }

    Ok(buf)
}
