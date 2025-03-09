use chrono::NaiveDate;

use crate::date_parser::DateParser;

pub struct EnDateParser {}

impl DateParser for EnDateParser {
    fn search_date(input: &str) -> Option<NaiveDate> {
        None
    }
}
