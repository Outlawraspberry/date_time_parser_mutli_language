use chrono::{Datelike, Days, Duration, Months, NaiveDate};
use regex::Regex;

use crate::{
    date_parser::DateParser,
    language::{
        shared::{DateExpression, DateFormat, Month},
        shared_date_parser::{parse_date_month_year, parse_month_date_year},
    },
    recognizable::Recognizable,
};

use super::expressions::{
    parse_date_in_week::parse_date_in_week, parse_day_alone::parse_day_alone,
    parse_in_n_months::parse_in_n_months, parse_keywords::parse_keywords,
    parse_month_date::parse_month_date, parse_relative_date::parse_relative_day,
    parse_relative_month::parse_relative_month,
};

/// Parsing a str into a `MonthOfYear` uses english abbreviations and full names.
impl Recognizable for Month {
    fn recognize(text: &str, _date_format: &DateFormat) -> Option<Month> {
        parse_month_of_year_english(text)
    }

    fn describe() -> &'static str {
        "month of year"
    }
}

/// Parses a `str` into an `Option` containing a `MonthOfYear`.
fn parse_month_of_year_english(text: &str) -> Option<Month> {
    let re = Regex::new(r"(?i)(?P<month>jan|january|feb|mar|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(month_match) = caps.name("month") {
            match month_match.as_str().to_lowercase().as_ref() {
                "jan" => return Some(Month::January),
                "feb" => return Some(Month::February),
                "mar" => return Some(Month::March),
                "apr" => return Some(Month::April),
                "may" => return Some(Month::May),
                "jun" => return Some(Month::June),
                "jul" => return Some(Month::July),
                "aug" => return Some(Month::August),
                "sep" => return Some(Month::September),
                "oct" => return Some(Month::October),
                "nov" => return Some(Month::November),
                "dec" => return Some(Month::December),
                _ => {}
            }
        }
    }
    None
}

pub struct EnDateParser {
    date_format: DateFormat,
}

impl Recognizable for DateExpression {
    fn recognize(input: &str, date_format: &DateFormat) -> Option<Self> {
        if let Some(date) = parse_keywords(input) {
            return Some(date);
        }

        if let Some(date) = parse_relative_day(input) {
            return Some(date);
        }

        match date_format {
            DateFormat::DayMonthYear => {
                if let Some(date) = parse_date_month_year(input) {
                    return Some(date);
                }
            }
            DateFormat::MonthDayYear => {
                if let Some(date) = parse_month_date_year(input) {
                    return Some(date);
                }
            }
        }
        
        if let Some(date) = parse_month_date(input, date_format) {
            return Some(date);
        }

        if let Some(date) = parse_date_in_week(input, date_format) {
            return Some(date);
        }
        
        if let Some(date) = parse_in_n_months(input) {
            return Some(date);
        }
        
        if let Some(date) = parse_relative_month(input) {
            return Some(date);
        }
        
        if let Some(date) = parse_day_alone(input) {
            return Some(date);
        }

        None
    }

    fn describe() -> &'static str {
        ""
    }
}

impl DateParser for EnDateParser {
    fn search_relative_date_expression(
        text: &str,
        now: &NaiveDate,
        date_format: &DateFormat,
    ) -> Option<NaiveDate> {
        if let Some(date_expr) = DateExpression::recognize(text, date_format) {
            match date_expr {
                DateExpression::InXDays(days) => {
                    return Some(now.checked_add_days(Days::new(days as u64)).unwrap());
                }

                DateExpression::DayInMonth(month, day) => {
                    let new_date = NaiveDate::from_ymd_opt(now.year(), month as u32, day);

                    match new_date {
                        Some(date) => {
                            if date.lt(&now) {
                                return date.checked_add_months(Months::new(12));
                            } else {
                                return Some(date);
                            }
                        }
                        _ => {
                            return None;
                        }
                    }
                }

                DateExpression::DayInMonth(m, d) => {
                    return NaiveDate::from_ymd_opt(now.year(), m as u32, d);
                }

                DateExpression::DayInMonthInYear(m, d, y) => {
                    return NaiveDate::from_ymd_opt(y, m as u32, d);
                }

                DateExpression::InXDays(n) => {
                    let d = Duration::days(n as i64);
                    return Some(now.checked_add_signed(d).unwrap());
                }

                DateExpression::DayInXWeeks(n, d) => {
                    let mut difference: i32 = (d.num_days_from_sunday() as i32)
                        - (now.weekday().num_days_from_sunday() as i32);
                    if difference < 0 {
                        difference += 7;
                    }
                    difference += 7 * (n as i32);
                    let dur = Duration::days(difference as i64);
                    return Some(now.checked_add_signed(dur).unwrap());
                }

                DateExpression::InXMonths(n) => {
                    let now_month = now.month();
                    let to_month = (now_month as i32) + n;
                    return NaiveDate::from_ymd_opt(now.year(), to_month as u32, now.day());
                }

                DateExpression::InMonthInYear(month, year) => {
                    return NaiveDate::from_ymd_opt(year, month as u32, 1);
                }
            }
        }
        None
    }
}
