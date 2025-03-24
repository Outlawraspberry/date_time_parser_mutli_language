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
- Added some tests

Modifications by Outlawraspberry UG (haftungsbeschränkt)

*/

use chrono::Weekday;
use regex::Regex;

use crate::language::shared::DateExpression;

/// Parses a `str` into an `Option` containing a `DateExpression::InWeek(i8, Weekday)`
pub fn parse_day_alone(text: &str) -> Option<DateExpression> {
    // saturday

    let re = Regex::new(r"(?i)(?P<day>mon|tue|wed|thu|fri|sat|sun)(r?day|r?sday|nesday|urday)?\b")
        .unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(day_match) = caps.name("day") {
            let d = day_match
                .as_str()
                .to_lowercase()
                .parse::<Weekday>()
                .unwrap();
            return Some(DateExpression::DayInXWeeks(0, d));
        }
    }

    None
}

#[cfg(test)]
mod parse_day_alone_works_when {
    use super::*;

    #[test]
    fn recognices_days() {
        assert_day("remind me on monday to do stuff", Weekday::Mon);
        assert_day("remind me on monday to do stuff", Weekday::Mon);
        assert_day("remind me on tuesday to do stuff", Weekday::Tue);
        assert_day("remind me on tue to do stuff", Weekday::Tue);
        assert_day("remind me on wednesday to do stuff", Weekday::Wed);
        assert_day("remind me on wed to do stuff", Weekday::Wed);
        assert_day("remind me on thursday to do stuff", Weekday::Thu);
        assert_day("remind me on thu to do stuff", Weekday::Thu);
        assert_day("remind me on friday to do stuff", Weekday::Fri);
        assert_day("remind me on fri to do stuff", Weekday::Fri);
        assert_day("remind me on saturday to do stuff", Weekday::Sat);
        assert_day("remind me on sat to do stuff", Weekday::Sat);
        assert_day("remind me on sunday to do stuff", Weekday::Sun);
        assert_day("remind me on sun to do stuff", Weekday::Sun);
    }

    fn assert_day(input: &str, expected_day: Weekday) {
        assert_eq!(
            parse_day_alone(input),
            Some(DateExpression::DayInXWeeks(0, expected_day)),
            "expected {} to be {:?}",
            input,
            expected_day
        );
    }
}
