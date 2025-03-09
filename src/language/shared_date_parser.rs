use chrono::{Datelike, Months, NaiveDate};
use regex::{Captures, Regex};

/// Parse date month year (dd.mm.yyyy) combinations out of strings.
///
/// Furthermore, the function can parse dd.mm combinations from strings.
/// To do this correclty, a current date is requred.
/// Dates in the past, will be calcualted for the next year.
pub fn parse_date_month_year(input: &str, now: NaiveDate) -> Option<NaiveDate> {
    let regex = Regex::new(r"\b(?<day>0?([1-9]|[12][0-9]|3[01]))\.(?<month>0?([1-9]|1[0-2]))(?:\.(?<year>\d{2}|\d{4}))?\b").unwrap();

    let Some(parts) = regex.captures(input) else {
        return None;
    };

    internal_parse_date(parts, now)
}

/// Parse month date year (mm.dd.yyyy) combinations out of strings.
///
/// Furthermore, the function can parse dd.mm combinations from strings.
/// To do this correclty, a current date is requred.
/// Dates in the past, will be calcualted for the next year.
pub fn parse_month_date_year(input: &str, now: NaiveDate) -> Option<NaiveDate> {
    let regex = Regex::new(r"\b(?<month>0?([1-9]|1[0-2]))\.(?<day>0?([1-9]|[12][0-9]|3[01]))(?:\.(?<year>\d{2}|\d{4}))?\b").unwrap();

    let Some(parts) = regex.captures(input) else {
        return None;
    };

    internal_parse_date(parts, now)
}

fn internal_parse_date(input: Captures, now: NaiveDate) -> Option<NaiveDate> {
    let year = if (input.name("year").is_some()) {
        input["year"].parse::<i32>().unwrap()
    } else {
        now.year()
    };
    let month = input["month"].parse::<u32>().unwrap();
    let day = input["day"].parse::<u32>().unwrap();

    let Some(date) = NaiveDate::from_ymd_opt(year, month, day) else {
        return None;
    };

    if date.lt(&now) {
        date.checked_add_months(Months::new(12))
    } else {
        Some(date)
    }
}

#[cfg(test)]
mod shared_data_parser_works_when {
    use crate::language::shared_date_parser::{parse_date_month_year, parse_month_date_year};
    use chrono::NaiveDate;

    fn get_fake_today_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2025, 4, 20).unwrap()
    }

    #[test]
    fn none_is_returned_when_no_date_was_found() {
        assert_eq!(parse_date_month_year("", get_fake_today_date()), None);
        assert_eq!(
            parse_date_month_year("No date found", get_fake_today_date()),
            None
        );
    }

    #[test]
    fn date_month_year_combinations_are_detected_correctly() {
        let test_date = NaiveDate::from_ymd_opt(2026, 12, 5).unwrap();

        assert_eq!(
            parse_date_month_year("5.12.2026", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_date_month_year("05.12.2026", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_date_month_year("Remember me on 5.12.2026", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_date_month_year("Remember me on 05.12.2026", get_fake_today_date()),
            Some(test_date)
        );
    }

    #[test]
    fn month_date_year_combinations_are_detected_correctly() {
        let test_date = NaiveDate::from_ymd_opt(2026, 12, 5).unwrap();

        assert_eq!(
            parse_month_date_year("12.5.2026", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_month_date_year("12.05.2026", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_month_date_year("Remember me on 12.5.2026", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_month_date_year("Remember me on 12.05.2026", get_fake_today_date()),
            Some(test_date)
        );
    }

    #[test]
    fn date_month_combinations_are_detected_correctly() {
        let test_date = NaiveDate::from_ymd_opt(2025, 12, 5).unwrap();

        assert_eq!(
            parse_date_month_year("5.12", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_date_month_year("05.12", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_date_month_year("Remember me on 5.12", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_date_month_year("Remember me on 05.12", get_fake_today_date()),
            Some(test_date)
        );
    }

    #[test]
    fn month_date_combinations_are_detected_correctly() {
        let test_date = NaiveDate::from_ymd_opt(2025, 12, 5).unwrap();

        assert_eq!(
            parse_month_date_year("12.5", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_month_date_year("12.05", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_month_date_year("Remember me on 12.5", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_month_date_year("Remember me on 12.05", get_fake_today_date()),
            Some(test_date)
        );
    }

    #[test]
    fn date_month_combination_for_current_year_is_over_calculated_to_next_year() {
        let test_date = NaiveDate::from_ymd_opt(2026, 3, 5).unwrap();

        assert_eq!(
            parse_date_month_year("5.3.", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_date_month_year("05.3", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_date_month_year("05.03", get_fake_today_date()),
            Some(test_date)
        );
    }

    #[test]
    fn month_date_combination_for_current_year_is_over_calculated_to_next_year() {
        let test_date = NaiveDate::from_ymd_opt(2026, 3, 5).unwrap();

        assert_eq!(
            parse_month_date_year("3.5.", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_month_date_year("3.05", get_fake_today_date()),
            Some(test_date)
        );
        assert_eq!(
            parse_month_date_year("03.05", get_fake_today_date()),
            Some(test_date)
        );
    }
}
