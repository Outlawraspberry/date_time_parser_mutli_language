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

use chrono::Weekday;
use regex::Regex;

use crate::{
    language::shared::{DateExpression, DateFormat},
    recognizable::Recognizable,
};

/// Parses a `str` into an `Option` containing a `DateExpression::InWeek(i8, Weekday)`
pub fn parse_date_in_week(text: &str, date_format: &DateFormat) -> Option<DateExpression> {
    // sat, this saturday, next saturday, last saturday, this sat,

    let re = Regex::new(r"(?i)(?P<prep>next|last|this)\s(?P<day>\w+)").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(prep_match) = caps.name("prep") {
            let relative_week = match prep_match.as_str().to_lowercase().as_ref() {
                "next" => 1,
                "last" => -1,
                "this" => 0,
                _ => 0,
            };

            if let Some(day_match) = caps.name("day") {
                let day_str = day_match.as_str();

                if let Some(day) = Weekday::recognize(day_str, date_format) {
                    return Some(DateExpression::DayInXWeeks(relative_week, day));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod parse_date_in_week_works_when {
    use super::{parse_date_in_week, DateExpression, DateFormat};
    use chrono::Weekday;

    #[test]
    fn day_in_n_weeks() {
        assert_day_in_n_weeks("next thursday", Weekday::Thu, 1);
        assert_day_in_n_weeks("last wed", Weekday::Wed, -1);
        assert_day_in_n_weeks("this monday", Weekday::Mon, 0);
        assert_day_in_n_weeks("next friday", Weekday::Fri, 1);
    }

    fn assert_day_in_n_weeks(text: &str, day: Weekday, relative_week: i32) {
        assert_eq!(
            parse_date_in_week(text, &DateFormat::DayMonthYear),
            Some(DateExpression::DayInXWeeks(relative_week, day))
        )
    }
}
