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
    fn into(&self, base_cfg: &CategoryMappingConfig) -> CategoryMappingConfig {
        CategoryMappingConfig {
            default: self
                .default
                .as_ref()
                .cloned()
                .unwrap_or_else(|| base_cfg.default.clone()),
            mapping: match self.mapping.as_ref() {
                Some(v) => {
                    let mut base = base_cfg.mapping.clone();
                    base.extend(v.iter().map(|(key, value)| (key.clone(), value.clone())));
                    base
                }
                None => base_cfg.mapping.clone(),
            },
        }
    }
}

#[derive(Deserialize)]
struct PayeeMapping {
    mapping: Option<HashMap<String, Vec<String>>>,
}

impl PayeeMapping {
    fn into(&self, base_cfg: &PayeeMappingConfig) -> PayeeMappingConfig {
        PayeeMappingConfig {
            mapping: match self.mapping.as_ref() {
                Some(v) => {
                    let mut base = base_cfg.mapping.clone();
                    base.extend(v.iter().map(|(key, value)| (key.clone(), value.clone())));
                    base
                }
                None => base_cfg.mapping.clone(),
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
                .cloned()
                .unwrap_or_else(|| base.income.clone()),
            category: self
                .category
                .as_ref()
                .map(|v| v.into(&base.category))
                .unwrap_or_else(|| base.category.clone()),
            payee: self
                .payee
                .as_ref()
                .map(|v| v.into(&base.payee))
                .unwrap_or_else(|| base.payee.clone()),
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
        assert_eq!(
            load_config("./test-data/cfg.conf".to_owned()),
            Ok(vec![
                (
                    "default".to_owned(),
                    BankConfig {
                        income: "Unknown".to_owned(),
                        category: CategoryMappingConfig {
                            default: "Unknown".to_owned(),
                            mapping: vec![(
                                "Family".to_owned(),
                                vec!["Joe Doe".to_owned(), "Jill Doe".to_owned()]
                            )]
                            .into_iter()
                            .collect()
                        },
                        payee: PayeeMappingConfig {
                            mapping: vec![(
                                "joe doe".to_owned(),
                                vec!["joe doe".to_owned(), "doe joe".to_owned()]
                            )]
                            .into_iter()
                            .collect()
                        }
                    }
                ),
                (
                    "bank1".to_owned(),
                    BankConfig {
                        income: "Banke Nr. 1".to_owned(),
                        category: CategoryMappingConfig {
                            default: "Unknown".to_owned(),
                            mapping: vec![("Family".to_owned(), vec!["Kill Bill".to_owned()])]
                                .into_iter()
                                .collect()
                        },
                        payee: PayeeMappingConfig {
                            mapping: vec![("joe doe".to_owned(), vec!["Kill Bill".to_owned()])]
                                .into_iter()
                                .collect()
                        }
                    }
                ),
                (
                    "bank2".to_owned(),
                    BankConfig {
                        income: "Banke Nr. 2".to_owned(),
                        category: CategoryMappingConfig {
                            default: "Whatever".to_owned(),
                            mapping: vec![
                                (
                                    "Family".to_owned(),
                                    vec!["Joe Doe".to_owned(), "Jill Doe".to_owned()]
                                ),
                                ("Friends".to_owned(), vec!["Kill Bill".to_owned()])
                            ]
                            .into_iter()
                            .collect()
                        },
                        payee: PayeeMappingConfig {
                            mapping: vec![(
                                "joe doe".to_owned(),
                                vec!["joe doe".to_owned(), "doe joe".to_owned()]
                            )]
                            .into_iter()
                            .collect()
                        }
                    }
                )
            ]
            .into_iter()
            .collect())
        );
    }
}
