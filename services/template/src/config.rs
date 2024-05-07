use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use anyhow::anyhow;

use serde::{Deserialize, Serialize};

use database::config::DatabaseConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateServiceConfig {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateConfig {
    pub database: DatabaseConfig,
    pub template: TemplateServiceConfig,
}

impl TemplateConfig {
    pub(crate) fn new(file_path: &Path) -> anyhow::Result<TemplateConfig> {
        let mut file = File::open(file_path)
            .map_err(|err| anyhow!("Can't open file {:?}: {}", file_path, err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| anyhow!("Can't read {:?}: {}", file_path, err))?;
        let config: TemplateConfig = serde_yaml::from_str(&contents)
            .map_err(|err| anyhow!("Can't read yaml {:?}: {}", file_path, err))?;
        Ok(config)
    }
}
