use regex::Regex;

use crate::language::shared::{DateExpression, DateFormat};

/// Parses a `str` into an `Option` containing a `DateExpression::DayInXWeeks(i8)`
pub fn parse_keyword_relative_week(text: &str, _date_format: &DateFormat) -> Option<DateExpression> {

    let re = Regex::new(r"(?i)(?P<prep>next|last|this)\s(?P<week>week)").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(prep_match) = caps.name("prep") {
            let relative_week = match prep_match.as_str().to_lowercase().as_ref() {
                "next" => 1,
                "last" => -1,
                "this" => 0,
                _ => 0,
            };

            return Some(DateExpression::DayInXWeeks(relative_week, chrono::Weekday::Mon));
        }
    }

    None
}

#[cfg(test)]
mod parse_keyword_based_relative_week {
    use super::{parse_keyword_relative_week, DateExpression, DateFormat};

    #[test]
    fn day_in_n_weeks() {
        assert_day_in_n_weeks("do something next week", 1);
        assert_day_in_n_weeks("do something last week", -1);
        assert_day_in_n_weeks("do something this week", 0);
    }

    fn assert_day_in_n_weeks(text: &str, relative_week: i8) {
        assert_eq!(
            parse_keyword_relative_week(text, &DateFormat::DayMonthYear),
            Some(DateExpression::DayInXWeeks(relative_week, chrono::Weekday::Mon))
        )
    }
}
