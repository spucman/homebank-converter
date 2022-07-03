use crate::{
    bank::{
        find_payee,
        homebank::{HomebankAccountingLine, IntoHomebankAccountingLine},
        look_for_mapping_in_text, switch_key_with_values_of_map, UNKNOWN_PAYEE,
    },
    config::BankConfig,
    data::{error::Error::*, Result},
};
use chrono::NaiveDate;
use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct BawagAccountingLine {
    pub iban: String,
    pub text: String,
    pub execution_date: NaiveDate,
    pub amount: f32,
    pub currency: String,
}

impl IntoHomebankAccountingLine for BawagAccountingLine {
    fn into(self, bank_cfg: &BankConfig) -> HomebankAccountingLine {
        let payees = switch_key_with_values_of_map(&bank_cfg.payee.mapping);
        let mut payee_keys: Vec<String> = payees.keys().cloned().collect();
        payee_keys.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

        let categories = switch_key_with_values_of_map(&bank_cfg.category.mapping);
        let mut category_keys: Vec<String> = categories.keys().cloned().collect::<Vec<String>>();
        category_keys.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

        let text_to_search = vec![self.text.clone()];

        HomebankAccountingLine {
            date: self.execution_date,
            memo: self.text.clone(),
            amount: self.amount,
            tags: vec![],
            payee: find_payee(
                &payees,
                &payee_keys,
                &text_to_search,
                self.amount,
                UNKNOWN_PAYEE.to_string(),
            )
            .unwrap_or_else(|| UNKNOWN_PAYEE.to_string()),
            category: look_for_mapping_in_text(&categories, &category_keys, &text_to_search)
                .unwrap_or_else(|| bank_cfg.category.default.to_owned()),
        }
    }
}

pub fn parse_csv(filepath: String) -> Result<Vec<BawagAccountingLine>> {
    let rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .double_quote(false)
        .from_path(filepath)
        .map_err(UnableToInitializeCSVReader)?;

    let result: Vec<BawagAccountingLine> = rdr
        .into_records()
        .filter_map(convert_to_accounting_line)
        .collect();

    Ok(result)
}

fn convert_to_accounting_line(
    record: std::result::Result<StringRecord, csv::Error>,
) -> Option<BawagAccountingLine> {
    match record {
        Ok(line) => Some(BawagAccountingLine {
            iban: line.get(0).unwrap_or("").to_string(),
            text: line.get(1).unwrap_or("").to_string(),
            execution_date: parse_to_date(line.get(2).unwrap_or("1.1.1900")),
            amount: parse_to_number(line.get(4).unwrap_or("0.0")),
            currency: line.get(5).unwrap_or("").to_string(),
        }),
        Err(e) => {
            println!("Unable to parse line: {}", e);
            None
        }
    }
}

fn parse_to_number(number: &str) -> f32 {
    number
        .replace('.', "")
        .replace(',', ".")
        .parse::<f32>()
        .unwrap()
}

fn parse_to_date(date: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(date, "%d.%m.%Y") {
        Ok(v) => v,
        Err(err) => {
            println!("{} - value: {}", err, date);
            panic!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSV_FILE: &str = "./test-data/bawag.csv";

    #[test]
    fn test_parse_csv() {
        assert_eq!(
            parse_csv(CSV_FILE.to_owned()).unwrap(),
            vec![
                BawagAccountingLine {
                    iban: "AT123456789".to_owned(),
                    amount: -15.39,
                    currency: "EUR".to_owned(),
                    text: "Some Text".to_owned(),
                    execution_date: NaiveDate::from_ymd(2020, 5, 26)
                },
                BawagAccountingLine {
                    iban: "AT123456789".to_owned(),
                    amount: -880.00,
                    currency: "EUR".to_owned(),
                    text: "Some Other Text".to_owned(),
                    execution_date: NaiveDate::from_ymd(2020, 5, 25)
                },
                BawagAccountingLine {
                    iban: "AT123456789".to_owned(),
                    amount: -2.40,
                    currency: "EUR".to_owned(),
                    text: "Some Text with Company GmbH\\\\SOME PLACE\\".to_owned(),
                    execution_date: NaiveDate::from_ymd(2020, 5, 25)
                }
            ]
        );
    }

    #[test]
    fn test_parse_to_number() {
        assert_eq!(parse_to_number("1.1"), 11.0);
        assert_eq!(parse_to_number("1,1"), 1.1);
    }

    #[test]
    fn test_parse_to_date() {
        assert_eq!(
            parse_to_date("25.05.2020"),
            NaiveDate::from_ymd(2020, 5, 25)
        );
    }
}
