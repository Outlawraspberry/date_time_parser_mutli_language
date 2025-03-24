use chrono::NaiveDate;

use crate::language::shared::DateFormat;

pub trait DateParser {
    fn search_relative_date_expression(
        input: &str,
        now: &NaiveDate,
        date_format: &DateFormat,
    ) -> Option<NaiveDate>;
}
