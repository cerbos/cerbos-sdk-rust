// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use anyhow::bail;
use serde::{Deserialize, Deserializer};

use crate::genpb::cerbos::{effect::v1::Effect, policy::v1::Policy};
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

fn mk_decoder(src: impl Read) -> anyhow::Result<Policy> {
    let mut buf = BufReader::with_capacity(BUF_SIZE, src);

    // Peek at the buffer to determine format
    let prelude = buf.fill_buf().map(String::from_utf8_lossy)?;
    let trimmed = prelude.trim_start();
    if trimmed.starts_with(JSON_START) {
        // read from JSON
        Box::new(new_json_decoder(buf))
    } else {
        let data = parse_yaml(buf)?;
    }
    bail!("not implemented")
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

fn parse_yaml<R: Read>(reader: BufReader<R>) -> Result<Vec<u8>, Box<dyn Error>> {
    let yaml_sep = "---";
    let yaml_comment = "#";
    let newline = b'\n';

    let mut buf = Vec::new();
    let mut num_docs = 0;
    let mut seen_content = false;

    for line_result in reader.lines() {
        let line = line_result.map_err(|e| format!("failed to read from source: {}", e))?;
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
                return Err(Box::new(MultipleYAMLDocsError));
            }
        }

        buf.extend_from_slice(line.as_bytes());
        buf.push(newline);
    }

    Ok(buf)
}
