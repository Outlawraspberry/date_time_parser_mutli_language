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
- Extends the regex to match the words `one` to `twelve`

Modifications by Outlawraspberry UG (haftungsbeschränkt)

*/

use regex::Regex;

use crate::language::shared::DateExpression;

/// Parses a `str` into an `Option` containing a `DateExpr::InNDays(i32)`
pub fn parse_relative_day(text: &str) -> Option<DateExpression> {
    // Define the regex pattern
    let re = Regex::new(r"(in\s(?P<num>([\d]{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\s(days?))").unwrap();

    // Check for captures
    if let Some(caps) = re.captures(&String::from(text).trim().to_lowercase()) {
        if let Some(num_match) = caps.name("num") {
            // Match numeric values
            if let Ok(num) = num_match.as_str().parse::<i32>() {
                return Some(DateExpression::InXDays(num));
            } else {
                // Match textual representations
                let num_str = num_match.as_str();
                let num = match num_str {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    "ten" => 10,
                    "eleven" => 11,
                    "twelve" => 12,
                    _ => return None, // Return None if it doesn't match
                };
                return Some(DateExpression::InXDays(num));
            }
        }
    }
    None
}

#[cfg(test)]
mod parse_relative_date_works_when {
    use crate::language::{
        en::expressions::parse_relative_date::parse_relative_day, shared::DateExpression,
    };

    #[test]
    fn expression_with_digit_is_found() {
        assert_in_n_days("Lunch in 6 days", 6);
        assert_in_n_days("Lunch in 1 day", 1);
        assert_in_n_days("Lunch in 300 days", 300);
    }

    #[test]
    fn test_parse_date_expression() {
        // Test cases for textual representations of numbers
        let cases = vec![
            ("in one days", Some(DateExpression::InXDays(1))),
            ("in One days", Some(DateExpression::InXDays(1))),
            ("in ONE days", Some(DateExpression::InXDays(1))),
            ("in two days", Some(DateExpression::InXDays(2))),
            ("in Two days", Some(DateExpression::InXDays(2))),
            ("in TWO days", Some(DateExpression::InXDays(2))),
            ("in three days", Some(DateExpression::InXDays(3))),
            ("in four days", Some(DateExpression::InXDays(4))),
            ("in five days", Some(DateExpression::InXDays(5))),
            ("in six days", Some(DateExpression::InXDays(6))),
            ("in seven days", Some(DateExpression::InXDays(7))),
            ("in eight days", Some(DateExpression::InXDays(8))),
            ("in nine days", Some(DateExpression::InXDays(9))),
            ("in ten days", Some(DateExpression::InXDays(10))),
            ("in eleven days", Some(DateExpression::InXDays(11))),
            ("in twelve days", Some(DateExpression::InXDays(12))),
        ];

        for (input, expected) in cases {
            let result = parse_relative_day(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_invalid_cases() {
        // Test cases for invalid inputs
        let invalid_cases = vec![
            "in zero days",
            "in thirteen days",
            "in days",
            "in one",
            "in twelve",
            "in twelve apples",
        ];

        for input in invalid_cases {
            let result = parse_relative_day(input);
            assert_eq!(result, None, "Expected None for input: {}", input);
        }
    }

    fn assert_in_n_days(text: &str, n: i32) {
        assert_eq!(parse_relative_day(text), Some(DateExpression::InXDays(n)))
    }
}
