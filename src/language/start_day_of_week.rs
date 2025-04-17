#[derive(PartialEq, Debug, Clone)]
pub enum StartDayOfWeek {
    Sunday,
    Monday,
}

/// Supported values are `monday` and `sunday`.
///
/// ```
/// use date_time_parser_multi_language::StartDayOfWeek;
//
/// fn main() {
///     assert_eq!(StartDayOfWeek::from("monday"), StartDayOfWeek::Monday);
///     assert_eq!(StartDayOfWeek::from("sunday"), StartDayOfWeek::Sunday);
/// }
/// ```
impl From<&str> for StartDayOfWeek {
    fn from(input: &str) -> StartDayOfWeek {
        if input == "sunday" {
            return StartDayOfWeek::Sunday;
        }
        if input == "monday" {
            return StartDayOfWeek::Monday;
        }

        StartDayOfWeek::Sunday
    }
}

#[cfg(test)]
mod from_str_works_for_start_day_of_week_when {
    use super::StartDayOfWeek;

    #[test]
    fn monday_is_parsed_correctly() {
        assert_eq!(StartDayOfWeek::from("monday"), StartDayOfWeek::Monday);
    }

    #[test]
    fn sunday_is_parsed_correctly() {
        assert_eq!(StartDayOfWeek::from("sunday"), StartDayOfWeek::Sunday);
    }
}
