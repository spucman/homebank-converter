use crate::config::{
    error::ConfigurationError::{self, *},
    hocon::load_config,
};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File as StdFile},
    io::prelude::*,
    path::Path,
    result::Result as StdResult,
};

pub mod error;
mod hocon;

const DEFAULT_FILE_NAME: &str = "config.conf";
const DEFAULT_DIR_CFG: &str = ".hbc";
const DEFAULT_BANK_CFG: &str = "default";

#[derive(Debug, Clone)]
pub struct CategoryMappingConfig {
    pub default: String,
    pub mapping: HashMap<String, Vec<String>>,
}

impl Default for CategoryMappingConfig {
    fn default() -> Self {
        CategoryMappingConfig {
            default: "Unknown".to_owned(),
            mapping: HashMap::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PayeeMappingConfig {
    pub mapping: HashMap<String, Vec<String>>,
}

impl Default for PayeeMappingConfig {
    fn default() -> Self {
        PayeeMappingConfig {
            mapping: HashMap::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BankConfig {
    pub income: String,
    pub category: CategoryMappingConfig,
    pub payee: PayeeMappingConfig,
}

impl Default for BankConfig {
    fn default() -> Self {
        BankConfig {
            income: "Unknown".to_owned(),
            category: CategoryMappingConfig::default(),
            payee: PayeeMappingConfig::default(),
        }
    }
}

pub struct Config {
    bankConfig: HashMap<String, BankConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let mut bankConfig = HashMap::<String, BankConfig>::with_capacity(1);
        bankConfig.insert(DEFAULT_BANK_CFG.to_owned(), BankConfig::default());
        Config {
            bankConfig: bankConfig,
        }
    }
}

impl Config {
    pub fn new(custom_path: Option<&str>) -> StdResult<Self, ConfigurationError> {
        let path = match custom_path {
            Some(v) => v.to_owned(),
            None => get_default_file_path(),
        };

        log::debug!("Trying to load config from path {}", path);
        if !Path::new(&path).exists() {
            log::warn!("No config file found - using default config");
            return Ok(Config::default());
        }

        Ok(Config {
            bankConfig: load_config(path)?,
        })
    }
}

fn get_default_cfg_dir() -> String {
    match home::home_dir() {
        Some(path) => format!("{}/{}", path.display(), DEFAULT_DIR_CFG),
        None => DEFAULT_DIR_CFG.to_owned(),
    }
}

fn get_default_file_path() -> String {
    format!("{}/{}", get_default_cfg_dir(), DEFAULT_FILE_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_add_activity_alias() {}
}
