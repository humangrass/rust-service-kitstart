use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use database::config::DatabaseConfig;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateServiceConfig {
    pub host: String,
    pub port: u32,
    pub debug: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateConfig {
    pub database: DatabaseConfig,
    pub template: TemplateServiceConfig,
}

impl TemplateConfig {
    pub(crate) fn new(file_path: Option<PathBuf>) -> Result<TemplateConfig, Box<dyn std::error::Error>> {
        if let Some(path) = file_path {
            let mut file = File::open(&path).map_err(|err| {
                format!("Can't open file {:?}: {}", path, err)
            })?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(|err| {
                format!("Can't read {:?}: {}", path, err)
            })?;
            let config: TemplateConfig = serde_yaml::from_str(&contents).map_err(|err| {
                format!("Can't read yaml {:?}: {}", path, err)
            })?;

            Ok(config)
        } else {
            Err("File path not provided".into())
        }
    }
}
