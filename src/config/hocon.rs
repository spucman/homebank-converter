use crate::config::{
    error::ConfigurationError::{self},
    BankConfig, CategoryMappingConfig, PayeeMappingConfig, DEFAULT_BANK_CFG,
};
use hocon::HoconLoader;
use serde::Deserialize;
use std::{collections::HashMap, result::Result as StdResult};

#[derive(Deserialize)]
struct CategoryMapping {
    default: Option<String>,
    mapping: Option<HashMap<String, Vec<String>>>,
}

impl CategoryMapping {
    fn into(&self, baseCfg: &CategoryMappingConfig) -> CategoryMappingConfig {
        CategoryMappingConfig {
            default: self
                .default
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or(baseCfg.default.clone()),
            mapping: match self.mapping.as_ref() {
                Some(v) => {
                    let mut base = baseCfg.mapping.clone();
                    base.extend(v.iter().map(|(key, value)| (key.clone(), value.clone())));
                    base
                }
                None => baseCfg.mapping.clone(),
            },
        }
    }
}

#[derive(Deserialize)]
struct PayeeMapping {
    mapping: Option<HashMap<String, Vec<String>>>,
}

impl PayeeMapping {
    fn into(&self, baseCfg: &PayeeMappingConfig) -> PayeeMappingConfig {
        PayeeMappingConfig {
            mapping: match self.mapping.as_ref() {
                Some(v) => {
                    let mut base = baseCfg.mapping.clone();
                    base.extend(v.iter().map(|(key, value)| (key.clone(), value.clone())));
                    base
                }
                None => baseCfg.mapping.clone(),
            },
        }
    }
}

#[derive(Deserialize)]
struct Bank {
    income: Option<String>,
    category: Option<CategoryMapping>,
    payee: Option<PayeeMapping>,
}

impl Bank {
    fn into(&self, base: &BankConfig) -> BankConfig {
        BankConfig {
            income: self
                .income
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or(base.income.clone()),
            category: self
                .category
                .as_ref()
                .map(|v| v.into(&base.category))
                .unwrap_or(base.category.clone()),
            payee: self
                .payee
                .as_ref()
                .map(|v| v.into(&base.payee))
                .unwrap_or(base.payee.clone()),
        }
    }
}

pub fn load_config(path: String) -> StdResult<HashMap<String, BankConfig>, ConfigurationError> {
    let raw_config: HashMap<String, Bank> = HoconLoader::new()
        .load_file(path)
        .map_err(ConfigurationError::Hocon)?
        .resolve()
        .map_err(ConfigurationError::Hocon)?;

    let default_cfg = match raw_config.get(DEFAULT_BANK_CFG) {
        Some(v) => v.into(&BankConfig::default()),
        None => BankConfig::default(),
    };

    let config = raw_config
        .iter()
        .map(|(key, bank)| {
            let bank_cfg = if key == DEFAULT_BANK_CFG {
                default_cfg.clone()
            } else {
                bank.into(&default_cfg)
            };

            (key.clone(), bank_cfg)
        })
        .collect();

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let test_cfg = load_config("./test-data/cfg.conf".to_owned());

        println!("{:?}", test_cfg);
    }
}
