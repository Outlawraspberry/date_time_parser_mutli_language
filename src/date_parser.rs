use chrono::NaiveDate;

use crate::language::shared::{DateFormat, StartDayOfWeek};

pub trait DateParser {
    fn search_relative_date_expression(
        input: &str,
        now: &NaiveDate,
        date_format: &DateFormat,
        start_of_week: &StartDayOfWeek,
    ) -> Option<NaiveDate>;
}
