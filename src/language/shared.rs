use chrono::Weekday;

use crate::recognizable::Recognizable;

pub enum DateFormat {
    DayMonthYear,
    MonthDayYear,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

pub fn num_to_month(num: u32) -> Option<Month> {
    match num {
        1 => Some(Month::January),
        2 => Some(Month::February),
        3 => Some(Month::March),
        4 => Some(Month::April),
        5 => Some(Month::May),
        6 => Some(Month::June),
        7 => Some(Month::July),
        8 => Some(Month::August),
        9 => Some(Month::September),
        10 => Some(Month::October),
        11 => Some(Month::November),
        12 => Some(Month::December),
        _ => None,
    }
}

/// Parsing a str into a `Weekday` uses the format %W.
impl Recognizable for Weekday {
    fn recognize(text: &str, _date_format: &DateFormat) -> Option<Weekday> {
        text.parse::<Weekday>().ok()
    }

    fn describe() -> &'static str {
        "day of week"
    }
}

#[derive(Debug, PartialEq, Clone)]
/// An abstract syntax for parsing dates.
pub enum DateExpression {
    InXDays(i32),
    DayInXWeeks(i8, Weekday), // e.g. next week monday => DayInXWeeks(1, Mon)
    InXMonths(i32),           // e.g. in 2 months => InXMonths(2)
    DayInMonth(Month, u32),   // e.g. June 8th => InMonth(Jun, 8)
    DayInMonthInYear(Month, u32, i32), // e.g. June 8th, 2019 => InYear(Jun, 8, 2019)
    InMonthInYear(Month, i32),
}

#[cfg(test)]
mod shared_components_work_when {
    use crate::language::shared::*;

    #[test]
    fn some_number_is_parsed_to_month() {
        assert_eq!(num_to_month(1), Some(Month::January));
        assert_eq!(num_to_month(2), Some(Month::February));
        assert_eq!(num_to_month(3), Some(Month::March));
        assert_eq!(num_to_month(4), Some(Month::April));
        assert_eq!(num_to_month(5), Some(Month::May));
        assert_eq!(num_to_month(6), Some(Month::June));
        assert_eq!(num_to_month(7), Some(Month::July));
        assert_eq!(num_to_month(8), Some(Month::August));
        assert_eq!(num_to_month(9), Some(Month::September));
        assert_eq!(num_to_month(10), Some(Month::October));
        assert_eq!(num_to_month(11), Some(Month::November));
        assert_eq!(num_to_month(12), Some(Month::December));
        assert_eq!(num_to_month(123), None);
    }
}
