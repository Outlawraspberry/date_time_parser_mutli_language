use chrono::NaiveDate;

use crate::{
    date_parser::DateParser,
    language::{
        shared::{DateExpression, DateFormat},
        shared_date_parser::{parse_date_month_year, parse_month_date_year},
    },
};

use super::expressions::{
    parse_date_in_week::parse_date_in_week, parse_day_alone::parse_day_alone, parse_in_n_months::parse_in_n_months, parse_keywords::parse_keywords, parse_month_date::parse_month_date, parse_relative_date::parse_relative_day, parse_relative_month::parse_relative_month
};

pub struct EnDateParser {}

impl DateParser for EnDateParser {
    fn search_relative_date_expression(
        input: &str,
        _now: NaiveDate,
        date_format: DateFormat,
    ) -> Option<DateExpression> {
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
        if let Some(date) = parse_month_date(input) {
            return Some(date);
        }
        if let Some(date) = parse_date_in_week(input) {
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
}
