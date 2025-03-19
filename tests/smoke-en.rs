use chrono::{Days, NaiveDate};
use date_time_parser_multi_language::{DateFormat, DateParser, EnDateParser};

#[test]
fn test_in_x_days() {
    let now = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;

    assert_in_x_days("in 5 days", 5, &now, &date_format);
    assert_in_x_days("tell me something in 10 days", 10, &now, &date_format);
    assert_in_x_days("something in 1 day", 1, &now, &date_format);
    assert_in_x_days("something is wrapped in 0 days in this sentence", 0, &now, &date_format);    
}


#[test]
fn test_keywords() {
    let now = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;

    assert_in_x_days("tell me something tomorrow", 1, &now, &date_format);
    assert_in_x_days("tell me somethin today", 0, &now, &date_format);
    // todo is this a case which breaks the program?
    // assert_in_x_days("tell me something yesterday", -1, &now, &date_format);
}

#[test]
fn test_relative_weeks() {
    let now = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;

    // todo add next week / relative keyword weeks
    //assert_in_x_weeks("Do something next week", 1, &now, &date_format); 
    //assert_in_x_weeks("Do something this week", 0, &now, &date_format); 

    assert_in_x_days("Do something in 2 week", 1 * 7, &now, &date_format);
    assert_in_x_days("Do something in 10 week", 10 * 7, &now, &date_format);
    assert_in_x_days("Do something in 100 week", 100 * 7, &now, &date_format);
}

#[test]
fn test_on_day_in_x_weeks() {
    // the first of december 2024 was a sunday
    let now = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
    let date_format = DateFormat::DayMonthYear;

    assert_in_x_days("Do something on monday next week", 1, &now, &date_format);
    assert_in_x_days("Do something on monday in two weeks", 8, &now, &date_format);
}


fn assert_in_x_days(input: &str, in_days: i32, now: &NaiveDate, date_format: &DateFormat) {
    let expected_date = now.checked_add_days(Days::new(in_days as u64)).unwrap();
    
    assert_eq!(EnDateParser::search_relative_date_expression(input, &now, date_format), Some(expected_date), "Failed to parse \"{}\" to {}", input, expected_date);
}
