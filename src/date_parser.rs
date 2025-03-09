use chrono::NaiveDate;

pub trait DateParser {
    fn search_date(input: &str) -> Option<NaiveDate>;
}
