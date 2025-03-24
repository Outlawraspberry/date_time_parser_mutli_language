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

use crate::language::shared::DateExpression;

/// Parses a `str` into an `Option` containing a `DateExpr::InNMonths(i32)`
pub fn parse_relative_month(text: &str) -> Option<DateExpression> {
    // this month, next month, last month
    let re = Regex::new(r"(?i)(?P<prep>next|last|this)\smonth").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(prep_match) = caps.name("prep") {
            let p = match prep_match.as_str().to_lowercase().as_ref() {
                "next" => 1,
                "last" => -1,
                "this" => 0,
                _ => 0,
            };

            return Some(DateExpression::InXMonths(p));
        }
    }

    None
}

#[cfg(test)]
mod parse_relative_month_works_when {
    use super::*;

    #[test]
    fn next_month_tests() {
        assert_relative_month("next month", 1);
        assert_relative_month("this month", 0);
        assert_relative_month("last month", -1);
    }

    fn assert_relative_month(text: &str, expected_n: i32) {
        assert_eq!(
            parse_relative_month(text),
            Some(DateExpression::InXMonths(expected_n))
        )
    }
}
