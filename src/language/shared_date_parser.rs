use regex::{Captures, Regex};

use super::shared::{num_to_month, DateExpression};

/// Parse date month year (dd.mm.yyyy) combinations out of strings.
///
/// Furthermore, the function can parse dd.mm combinations from strings.
/// To do this correclty, a current date is requred.
/// Dates in the past, will be calcualted for the next year.
pub fn parse_date_month_year(input: &str) -> Option<DateExpression> {
    let regex = Regex::new(r"\b(?:(?P<day>0?[1-9]|[12][0-9]|3[01])[-./](?P<month>0?[1-9]|1[0-2])(?:[-./](?P<year>\d{4}))?|(?P<month2>0?[1-9]|1[0-2])[-./](?P<year2>\d{4}))\b").unwrap();

    let Some(parts) = regex.captures(input) else {
        return None;
    };

    internal_parse_date(parts)
}

/// Parse month date year (mm.dd.yyyy) combinations out of strings.
///
/// Furthermore, the function can parse dd.mm combinations from strings.
/// To do this correclty, a current date is requred.
/// Dates in the past, will be calcualted for the next year.
pub fn parse_month_date_year(input: &str) -> Option<DateExpression> {
    let regex = Regex::new(r"\b(?:(?P<month>0?[1-9]|1[0-2])[-./](?P<day>0?[1-9]|[12][0-9]|3[01])(?:[-./](?P<year>\d{4}))?|(?P<month2>0?[1-9]|1[0-2])[-./](?P<year2>\d{4}))\b").unwrap();

    let Some(parts) = regex.captures(input) else {
        return None;
    };

    internal_parse_date(parts)
}

fn internal_parse_date(input: Captures) -> Option<DateExpression> {
    let year = if input.name("year").is_some() {
        Some(input["year"].parse::<i32>().unwrap())
    } else if input.name("year2").is_some() {
        Some(input["year2"].parse::<i32>().unwrap())
    } else {
        None
    };

    let month = if input.name("month").is_some() {
        Some(num_to_month(input["month"].parse::<u32>().unwrap()).unwrap())
    } else if input.name("month2").is_some() {
        Some(num_to_month(input["month2"].parse::<u32>().unwrap()).unwrap())
    } else {
        None
    };

    let day = if input.name("day").is_some() {
        Some(input["day"].parse::<u32>().unwrap())
    } else {
        None
    };

    if year.is_some() && month.is_some() && day.is_some() {
        return Some(DateExpression::DayInMonthInYear(
            month.unwrap(),
            day.unwrap(),
            year.unwrap(),
        ));
    } else if year.is_some() && month.is_some() {
        return Some(DateExpression::InMonthInYear(month.unwrap(), year.unwrap()));
    } else if month.is_some() && day.is_some() {
        return Some(DateExpression::DayInMonth(month.unwrap(), day.unwrap()));
    }

    None
}

#[cfg(test)]
mod shared_data_parser_works_when {
    use crate::language::{
        shared::{DateExpression, Month},
        shared_date_parser::{parse_date_month_year, parse_month_date_year},
    };

    #[test]
    fn none_is_returned_when_no_date_was_found() {
        assert_date_month_year("", None);
        assert_date_month_year("No date found", None);
    }

    #[test]
    fn date_month_year_combinations_are_detected_correctly() {
        let test_date = DateExpression::DayInMonthInYear(Month::December, 5, 2026);

        assert_date_month_year("5.12.2026", Some(test_date.clone()));
        assert_date_month_year("05.12.2026", Some(test_date.clone()));
        assert_date_month_year("Remind me on 5.12.2026", Some(test_date.clone()));
        assert_date_month_year("Remind me on 05.12.2026", Some(test_date.clone()));
    }

    #[test]
    fn month_date_year_combinations_are_detected_correctly() {
        let test_date = DateExpression::DayInMonthInYear(Month::December, 5, 2026);

        assert_month_date_year("12.5.2026", Some(test_date.clone()));
        assert_month_date_year("12.05.2026", Some(test_date.clone()));
        assert_month_date_year("Remind me on 12.5.2026", Some(test_date.clone()));
        assert_month_date_year("Remind me on 12.05.2026", Some(test_date.clone()));
    }

    #[test]
    fn date_month_combinations_are_detected_correctly() {
        let test_date = DateExpression::DayInMonth(Month::December, 5);

        assert_date_month_year("5.12", Some(test_date.clone()));
        assert_date_month_year("05.12", Some(test_date.clone()));
        assert_date_month_year("Remind me on 5.12", Some(test_date.clone()));
        assert_date_month_year("Remind me on 05.12", Some(test_date.clone()));
    }

    #[test]
    fn month_date_combinations_are_detected_correctly() {
        let test_date = DateExpression::DayInMonth(Month::December, 5);

        assert_month_date_year("12.5", Some(test_date.clone()));
        assert_month_date_year("12.05", Some(test_date.clone()));
        assert_month_date_year("Remind me on 12.5", Some(test_date.clone()));
        assert_month_date_year("Remind me on 12.05", Some(test_date.clone()));
    }

    fn assert_date_month_year(input: &str, expects: Option<DateExpression>) {
        assert_eq!(
            parse_date_month_year(input),
            expects,
            "Failed for input: {:?} - {:?}",
            input,
            expects
        )
    }

    fn assert_month_date_year(input: &str, expects: Option<DateExpression>) {
        assert_eq!(
            parse_month_date_year(input),
            expects,
            "Failed for input: {:?} - {:?}",
            input,
            expects
        )
    }
}
