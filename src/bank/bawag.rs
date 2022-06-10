use super::homebank::HomebankAccountingLine;
use crate::data::error::Error::*;
use crate::data::Result;
use chrono::NaiveDate;
use csv::ReaderBuilder;
use csv::StringRecord;

#[derive(Clone, Debug, PartialEq)]
pub struct BawagAccountingLine {
    pub iban: String,
    pub text: String,
    pub execution_date: NaiveDate,
    pub amount: f32,
    pub currency: String,
}

impl From<HomebankAccountingLine> for BawagAccountingLine {
    fn from(line: HomebankAccountingLine) -> Self {
        HomebankAccountingLine {
            
        }
    }
}

pub fn parse_csv(filepath: String) -> Result<Vec<BawagAccountingLine>> {
    let rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .double_quote(false)
        .from_path(filepath)
        .map_err(|e| UnableToInitializeCSVReader(e))?;

    let result: Vec<BawagAccountingLine> = rdr
        .into_records()
        .filter_map(|r| convert_to_accounting_line(r))
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
        .replace(".", "")
        .replace(",", ".")
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
