use chrono::{Days, NaiveDate};
use date_time_parser_multi_language::{DateFormat, DateParser, EnDateParser, StartDayOfWeek};

#[test]
fn test_in_x_days() {
    let now = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;
    let start_of_week = StartDayOfWeek::Monday;

    assert_in_x_days("in 5 days", 5, &now, &date_format, &start_of_week);
    assert_in_x_days("tell me something in 10 days", 10, &now, &date_format, &start_of_week);
    assert_in_x_days("something in 1 day", 1, &now, &date_format, &start_of_week);
    assert_in_x_days(
        "something is wrapped in 0 days in this sentence",
        0,
        &now,
        &date_format,
        &start_of_week
    );
}

#[test]
fn test_keywords() {
    let now = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;
    let start_of_week = StartDayOfWeek::Monday;

    assert_in_x_days("tell me something tomorrow", 1, &now, &date_format, &start_of_week);
    assert_in_x_days("tell me somethin today", 0, &now, &date_format, &start_of_week);
    // todo is this a case which breaks the program?
    // assert_in_x_days("tell me something yesterday", -1, &now, &date_format);
}

#[test]
fn test_relative_weeks() {
    // the first of december 2024 was a sunday
    let now = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;
    let start_of_week = StartDayOfWeek::Monday;

    // todo introduce start of the week.
    // 8 is the correct amount of days, because atm. it is expected that the week starts on sunday
    assert_in_x_days("Do something next week", 1, &now, &date_format, &start_of_week);
    assert_in_x_days("Do something this week", 1, &now, &date_format, &start_of_week);

    assert_in_x_days("Do something in 2 weeks", 2 * 7, &now, &date_format, &start_of_week);
    assert_in_x_days("Do something in 10 weeks", 10 * 7, &now, &date_format, &start_of_week);
    assert_in_x_days("Do something in 100 weeks", 100 * 7, &now, &date_format, &start_of_week);
}

#[test]
fn test_on_day_in_x_weeks() {
    // the first of december 2024 was a sunday
    let now = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;
    let start_of_week = StartDayOfWeek::Monday;

    assert_in_x_days("Do something on monday next week", 8, &now, &date_format, &start_of_week);
    assert_in_x_days(
        "Do something on monday in two weeks",
        14,
        &now,
        &date_format,
        &start_of_week
    );
    assert_in_x_days(
        "Do something on monday in 120 weeks",
        7 * 120,
        &now,
        &date_format,
        &start_of_week
    );
}

fn assert_in_x_days(input: &str, in_days: i32, now: &NaiveDate, date_format: &DateFormat, start_of_week: &StartDayOfWeek) {
    let expected_date = now.checked_add_days(Days::new(in_days as u64)).unwrap();

    assert_eq!(
        EnDateParser::search_relative_date_expression(input, &now, date_format, &start_of_week),
        Some(expected_date),
        "Failed to parse \"{}\" to {}",
        input,
        expected_date
    );
}

#[test]
fn test_a_specific_date_month() {
    let now = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;

    assert_specific_date(
        "Do something on 10th of december",
        NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
        &now,
        &date_format,
    );
    assert_specific_date(
        "Do something on 10th december",
        NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
        &now,
        &date_format,
    );
    assert_specific_date(
        "Do something on 10 december",
        NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
        &now,
        &date_format,
    );
    assert_specific_date(
        "Do something on 10.12",
        NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
        &now,
        &date_format,
    );
    // todo is "10th" a case? it has the implicit information of the month
    // assert_specific_date(
    //     "Do something on 10th",
    //     NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
    //     &now,
    //     &date_format
    // );
}

#[test]
fn test_a_specific_month_date() {
    let now = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;

    assert_specific_date(
        "Do something on 10th of december",
        NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
        &now,
        &date_format,
    );
    assert_specific_date(
        "Do something on 10th december",
        NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
        &now,
        &date_format,
    );
    assert_specific_date(
        "Do something on 10 december",
        NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
        &now,
        &date_format,
    );
    assert_specific_date(
        "Do something on 10.12",
        NaiveDate::from_ymd_opt(2024, 12, 10).unwrap(),
        &now,
        &date_format,
    );
}

#[test]
fn test_a_specific_date_month_year() {
    let now = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;

    assert_specific_date(
        "Do something on 10.12.2025",
        NaiveDate::from_ymd_opt(2025, 12, 10).unwrap(),
        &now,
        &date_format,
    );

    // todo is this a case?
    // assert_specific_date(
    //     "Do something on 10th of december 2025",
    //     NaiveDate::from_ymd_opt(2025, 12, 10).unwrap(),
    //     &now,
    //     &date_format
    // );
}

#[test]
fn test_a_specifig_month_date_year() {
    let now = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
    let date_format = DateFormat::MonthDayYear;

    assert_specific_date(
        "Do something on 12.10.2025",
        NaiveDate::from_ymd_opt(2025, 12, 10).unwrap(),
        &now,
        &date_format,
    );
}

fn assert_specific_date(
    input: &str,
    expected_date: NaiveDate,
    now: &NaiveDate,
    date_format: &DateFormat,
) {
    let start_of_week = StartDayOfWeek::Monday;

    assert_eq!(
        EnDateParser::search_relative_date_expression(input, now, date_format, &start_of_week),
        Some(expected_date),
        "failed to parse ${} as {}",
        input,
        expected_date
    );
}
