#[derive(PartialEq, Debug, Clone)]
pub enum DateFormat {
    DayMonthYear,
    MonthDayYear,
}

/// Supported values are `dayMonthYear` and `monthDayYear`.
///
/// ```
/// use date_time_parser_multi_language::DateFormat;
//
/// fn main() {
///     assert_eq!(DateFormat::from("dayMonthYear"), DateFormat::DayMonthYear);
///     assert_eq!(DateFormat::from("monthDayYear"), DateFormat::MonthDayYear);
/// }
/// ```
impl From<&str> for DateFormat {
    fn from(input: &str) -> DateFormat {
        if input == "dayMonthYear" {
            return DateFormat::DayMonthYear;
        }

        if input == "monthDayYear" {
            return DateFormat::MonthDayYear;
        }

        DateFormat::MonthDayYear
    }
}

#[cfg(test)]
mod from_str_works_for_date_format_when {
    use super::DateFormat;

    #[test]
    fn day_month_year_is_parsed_correctly() {
        assert_eq!(DateFormat::from("dayMonthYear"), DateFormat::DayMonthYear)
    }

    #[test]
    fn month_day_year_is_parsed_correctly() {
        assert_eq!(DateFormat::from("monthDayYear"), DateFormat::MonthDayYear)
    }
}
