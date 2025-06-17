// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::collections::HashMap;

use certs::CerbosTestTlsConfig;
use testcontainers::{
    core::{ContainerPort, Mount, WaitFor},
    Image,
};

pub mod certs;

pub struct CerbosContainer {
    image_name: String,
    image_tag: String,
    volume_mounts: Vec<Mount>,
    environment_vars: HashMap<String, String>,
    ports: Vec<ContainerPort>,
    cmd: Vec<String>,
}

impl Default for CerbosContainer {
    fn default() -> Self {
        let environment_vars =
            HashMap::from([("CERBOS_NO_TELEMETRY".to_string(), "1".to_string())]);
        Self {
            image_name: "ghcr.io/cerbos/cerbos".to_string(),
            image_tag: "latest".to_string(),
            volume_mounts: vec![],
            environment_vars,
            ports: vec![3592.into(), 3593.into()],
            cmd: vec![],
        }
    }
}

impl CerbosContainer {
    pub fn with_image_name<S>(self, image_name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            image_name: image_name.into(),
            ..self
        }
    }
    const CERT_PATH: &str = "/certs/";
    pub fn with_tls_config(self, config: &CerbosTestTlsConfig) -> Self {
        let mut mounts = vec![Mount::bind_mount(
            config.get_temp_dir().path().to_string_lossy(),
            Self::CERT_PATH,
        )];
        self.volume_mounts.into_iter().for_each(|x| mounts.push(x));
        Self {
            volume_mounts: mounts,
            cmd: vec![
                "server".to_string(),
                "--set=server.tls.cert=".to_string()
                    + Self::CERT_PATH
                    + CerbosTestTlsConfig::CERT_NAME,
                "--set=server.tls.key=".to_string()
                    + Self::CERT_PATH
                    + CerbosTestTlsConfig::CERT_KEY,
            ],
            ..self
        }
    }
    pub fn with_image_tag<S>(self, image_tag: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            image_tag: image_tag.into(),
            ..self
        }
    }

    pub fn with_volume_mounts<S, I>(self, mounts: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = (S, S)>,
    {
        Self {
            volume_mounts: mounts
                .into_iter()
                .map(|(k, v)| Mount::bind_mount(k.into(), v.into()))
                .collect(),
            ..self
        }
    }

    pub fn with_environment_vars<S, I>(self, vars: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = (S, S)>,
    {
        Self {
            environment_vars: vars
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..self
        }
    }
}

impl Image for CerbosContainer {
    fn name(&self) -> &str {
        &self.image_name
    }

    fn tag(&self) -> &str {
        &self.image_tag
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &self.ports
    }

    fn mounts(&self) -> impl IntoIterator<Item = &testcontainers::core::Mount> {
        &self.volume_mounts
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        self.environment_vars.iter()
    }

    fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
        vec![WaitFor::message_on_stdout("Starting HTTP server")]
    }
    fn cmd(&self) -> impl IntoIterator<Item = impl Into<Cow<'_, str>>> {
        self.cmd.iter()
    }
}
