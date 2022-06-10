use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq)]
pub struct HomebankAccountingLine {
    pub date: NaiveDate,
    pub payment: i32,
    pub info: Option<String>,
    pub payee: String,
    pub memo: String,
    pub amount: f32,
    pub category: String,
    pub tags: Vec<String>,
}
