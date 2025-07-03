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
};
use std::io::{self, BufRead, BufReader, Read};

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
const NEWLINE: u8 = b'\n';

static JSON_START: &str = "{";

trait Index {
    fn get(self: &Self, n: &str) -> Option<&Self>;
}

impl Index for serde_json::Value {
    fn get(self: &Self, n: &str) -> Option<&Self> {
        self.get(n)
    }
}
impl Index for serde_yml::Value {
    fn get(self: &Self, n: &str) -> Option<&Self> {
        self.get(n)
    }
}

trait Serde {
    type Value: Index + Clone;
    fn from_value<T: serde::de::DeserializeOwned>(&self, value: Self::Value) -> anyhow::Result<T>;
    fn from_self<T: serde::de::DeserializeOwned>(self) -> anyhow::Result<T>;
    fn get(&self, name: &str) -> Option<&Self::Value>;
}

struct JsonSerde(serde_json::Value);
impl JsonSerde {
    fn new(buf: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Self(
            serde_json::from_slice(&buf).with_context(|| "fail to deser from slice")?,
        ))
    }
}
impl Serde for JsonSerde {
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

struct YamlSerde(serde_yml::Value);
impl YamlSerde {
    fn new(buf: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Self(
            serde_yml::from_slice(&buf).with_context(|| "fail to deser from slice")?,
        ))
    }
}
impl Serde for YamlSerde {
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
fn make_policy(ser: impl Serde) -> anyhow::Result<Policy> {
    let pt = if let Some(v) = ser.get("resourcePolicy") {
        let p: ResourcePolicy = ser.from_value(v.clone())?;
        Some(PolicyType::ResourcePolicy(p))
    } else if let Some(v) = ser.get("principalPolicy") {
        let p: PrincipalPolicy = ser.from_value(v.clone())?;
        Some(PolicyType::PrincipalPolicy(p))
    } else if let Some(v) = ser.get("derivedRoles") {
        let p: DerivedRoles = ser.from_value(v.clone())?;
        Some(PolicyType::DerivedRoles(p))
    } else if let Some(v) = ser.get("exportVariables") {
        let p: ExportVariables = ser.from_value(v.clone())?;
        Some(PolicyType::ExportVariables(p))
    } else if let Some(v) = ser.get("exportConstants") {
        let p: ExportConstants = ser.from_value(v.clone())?;
        Some(PolicyType::ExportConstants(p))
    } else if let Some(v) = ser.get("rolePolicy") {
        let p: RolePolicy = ser.from_value(v.clone())?;
        Some(PolicyType::RolePolicy(p))
    } else {
        None
    };
    let mut policy: Policy = ser.from_self()?;
    policy.policy_type = pt;
    Ok(policy)
}
pub fn read_policy(src: impl Read) -> anyhow::Result<Policy> {
    let mut buf = BufReader::with_capacity(BUF_SIZE, src);

    // Peek at the buffer to determine format
    let prelude = buf.fill_buf().map(String::from_utf8_lossy)?;
    let trimmed = prelude.trim_start();
    if trimmed.starts_with(JSON_START) {
        let mut h = buf.take(MAX_FILE_SIZE as u64);
        let mut data = Vec::new();
        h.read(&mut data)?;
        let ser = JsonSerde::new(data);
        use serde_json::Value;
        let value: Value = serde_json::from_slice(&data)?;
        let pt = if let Some(v) = value.get("resourcePolicy") {
            let p: ResourcePolicy = serde_json::from_value(v.clone())?;
            Some(PolicyType::ResourcePolicy(p))
        } else if let Some(v) = value.get("principalPolicy") {
            let p: PrincipalPolicy = serde_json::from_value(v.clone())?;
            Some(PolicyType::PrincipalPolicy(p))
        } else if let Some(v) = value.get("derivedRoles") {
            let p: DerivedRoles = serde_json::from_value(v.clone())?;
            Some(PolicyType::DerivedRoles(p))
        } else if let Some(v) = value.get("exportVariables") {
            let p: ExportVariables = serde_json::from_value(v.clone())?;
            Some(PolicyType::ExportVariables(p))
        } else if let Some(v) = value.get("exportConstants") {
            let p: ExportConstants = serde_json::from_value(v.clone())?;
            Some(PolicyType::ExportConstants(p))
        } else if let Some(v) = value.get("rolePolicy") {
            let p: RolePolicy = serde_json::from_value(v.clone())?;
            Some(PolicyType::RolePolicy(p))
        } else {
            None
        };
        let mut policy: Policy = serde_json::from_value(value)?;
        policy.policy_type = pt;
        Ok(policy)
    } else {
        use serde_yml::Value;
        let data: Vec<u8> = parse_yaml(buf)?;
        let value: Value = serde_yml::from_slice(&data)?;
        let pt = if let Some(v) = value.get("resourcePolicy") {
            let p: ResourcePolicy = serde_yml::from_value(v.clone())?;
            Some(PolicyType::ResourcePolicy(p))
        } else if let Some(v) = value.get("principalPolicy") {
            let p: PrincipalPolicy = serde_yml::from_value(v.clone())?;
            Some(PolicyType::PrincipalPolicy(p))
        } else if let Some(v) = value.get("derivedRoles") {
            let p: DerivedRoles = serde_yml::from_value(v.clone())?;
            Some(PolicyType::DerivedRoles(p))
        } else if let Some(v) = value.get("exportVariables") {
            let p: ExportVariables = serde_yml::from_value(v.clone())?;
            Some(PolicyType::ExportVariables(p))
        } else if let Some(v) = value.get("exportConstants") {
            let p: ExportConstants = serde_yml::from_value(v.clone())?;
            Some(PolicyType::ExportConstants(p))
        } else if let Some(v) = value.get("rolePolicy") {
            let p: RolePolicy = serde_yml::from_value(v.clone())?;
            Some(PolicyType::RolePolicy(p))
        } else {
            None
        };
        let mut policy: Policy = serde_yml::from_value(value)?;
        policy.policy_type = pt;
        Ok(policy)
    }
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MultipleYAMLDocsError;

impl fmt::Display for MultipleYAMLDocsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "more than one YAML document detected")
    }
}

impl Error for MultipleYAMLDocsError {}

fn parse_yaml<R: Read>(reader: BufReader<R>) -> anyhow::Result<Vec<u8>> {
    let yaml_sep = "---";
    let yaml_comment = "#";
    let newline = b'\n';

    let mut buf = Vec::new();
    let mut num_docs = 0;
    let mut seen_content = false;

    for line_result in reader.lines() {
        let line = line_result.with_context(|| format!("failed to read from source"))?;
        let trimmed_line = line.trim_start();

        // ignore comments
        if trimmed_line.starts_with(yaml_comment) {
            continue;
        }

        // ignore empty lines at the beginning of the file
        if !seen_content && trimmed_line.is_empty() {
            continue;
        }
        seen_content = true;

        if line.starts_with(yaml_sep) {
            num_docs += 1;
            if num_docs > 1 || (num_docs == 1 && !buf.is_empty()) {
                bail!("multiple YAML docs")
            }
        }

        buf.extend_from_slice(line.as_bytes());
        buf.push(newline);
    }

    Ok(buf)
}
