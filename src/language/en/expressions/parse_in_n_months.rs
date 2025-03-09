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
- Update regex to match more dates
- Add more tests to check the behaviors

Modifications by Outlawraspberry UG (haftungsbeschränkt)

*/

use regex::Regex;

use crate::language::shared::DateExpression;


/// Parses a `str` into an `Option` containing a `DateExpr::InNMonths(i32)`
pub fn parse_in_n_months(text: &str) -> Option<DateExpression> {
    // in 2 months

    let re = Regex::new(r"(in\s(?P<num>\d{1,3})\s(months?))").unwrap();
    if let Some(caps) = re.captures(text) {
        if let Some(num_match) = caps.name("num") {
            let num: i32 = num_match.as_str().parse().unwrap();
            return Some(DateExpression::InXMonths(num));
        }
    }

    None
}

#[cfg(test)]
mod parse_in_n_months_works_when {
    use super::*;

    #[test]
    fn relative_month_tests() {
        assert_relative_month("in 4 months", 4);
        assert_relative_month("in 1 month", 1);
    }

    fn assert_relative_month(text: &str, expected_n: i32) {
        assert_eq!(
            parse_in_n_months(text),
            Some(DateExpression::InXMonths(expected_n))
        )
    }
}