/*
Original source https://github.com/isaacrlee/event-parser/blob/9b05f16d0728b0178ffe58e7d65aa3ed8c857170/date_time_parser/src/date_parse.rs#L199

Original License:

MIT License

Copyright (c) 2020 Isaac Lee and Alex Grimes

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Modifications:

- Copied in separate file
- Used own date expression type

Modifications by Outlawraspberry UG (haftungsbeschränkt)

*/

use regex::Regex;

use crate::language::shared::{DateExpression, DateFormat, Month};
use crate::recognizable::Recognizable;

/// Parses a `str` into an `Option` containing a `DateExpr::InMonth(MonthOfYear, u32)`.
pub fn parse_month_date(text: &str, date_format: &DateFormat) -> Option<DateExpression> {
    //june 1, june 1st

    let re = Regex::new(r"(?i)(?P<date>\d{1,2})?(th)?\s(?P<month>jan|january|feb|mar|april|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)|(?P<month2>jan|january|feb|mar|april|may|jun|jul|aug|sep|oct|nov|dec)(r?uary|ch|il|e|y|ust|tember|ober|ember|\b)\s(?P<date2>\d{1,2})?(th)?").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(month_match) = caps.name("month").or(caps.name("month2")) {
            if let Some(date_match) = caps.name("date").or(caps.name("date2")) {
                let date: u32 = date_match.as_str().parse().unwrap();
                let month = month_match.as_str();
                if let Some(m) = Month::recognize(month, date_format) {
                    return Some(DateExpression::DayInMonth(m, date));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod parse_month_date_english_works_when {
    use super::{parse_month_date, DateExpression, DateFormat, Month};

    #[test]
    fn absolute_english_date_tests_month_day() {
        assert_recognize_in_month("Jun 15", Month::June, 15);
        assert_recognize_in_month("June 5th", Month::June, 5);
        assert_recognize_in_month("June 5", Month::June, 5);

        assert_recognize_in_month("Jan 15", Month::January, 15);
        assert_recognize_in_month("February 5th", Month::February, 5);
        assert_recognize_in_month("May 25", Month::May, 25);
    }

    #[test]
    fn absolute_english_date_tests_day_month() {
        assert_recognize_in_month("15 Jun ", Month::June, 15);
        assert_recognize_in_month("5th June ", Month::June, 5);
        assert_recognize_in_month("5 June ", Month::June, 5);

        assert_recognize_in_month("15 Jan", Month::January, 15);
        assert_recognize_in_month("5th February", Month::February, 5);
        assert_recognize_in_month("5th of February", Month::February, 5);
        assert_recognize_in_month("25 May", Month::May, 25);
    }

    fn assert_recognize_in_month(text: &str, expected_m: Month, expected_d: u32) {
        let date_expression = DateExpression::DayInMonth(expected_m, expected_d);

        assert_eq!(
            parse_month_date(text, &DateFormat::DayMonthYear),
            Some(date_expression.clone()),
            "Failed to parse {} to {:?}",
            text,
            date_expression
        )
    }
}
