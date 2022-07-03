use crate::config::BankConfig;
use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq)]
pub struct HomebankAccountingLine {
    pub date: NaiveDate,
    pub payee: String,
    pub memo: String,
    pub amount: f32,
    pub category: String,
    pub tags: Vec<String>,
}

pub trait IntoHomebankAccountingLine {
    fn into(self, bank_cfg: &BankConfig) -> HomebankAccountingLine;
}
