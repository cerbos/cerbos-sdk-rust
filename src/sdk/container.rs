use std::collections::HashMap;

use testcontainers::{core::WaitFor, Image};

pub struct CerbosContainer {
    image_name: String,
    image_tag: String,
    volume_mounts: HashMap<String, String>,
    environment_vars: HashMap<String, String>,
}

impl Default for CerbosContainer {
    fn default() -> Self {
        let mut environment_vars = HashMap::new();
        environment_vars.insert("CERBOS_NO_TELEMETRY".to_string(), "1".to_string());

        Self {
            image_name: "ghcr.io/cerbos/cerbos".to_string(),
            image_tag: "latest".to_string(),
            volume_mounts: HashMap::new(),
            environment_vars,
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
                .map(|(k, v)| (k.into(), v.into()))
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
    type Args = ();

    fn name(&self) -> String {
        self.image_name.to_string()
    }

    fn tag(&self) -> String {
        self.image_tag.to_string()
    }

    fn expose_ports(&self) -> Vec<u16> {
        vec![3592, 3593]
    }

    fn volumes(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.volume_mounts.iter())
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.environment_vars.iter())
    }

    fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
        vec![WaitFor::message_on_stdout("Starting HTTP server")]
    }
}
