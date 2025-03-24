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

/// Parses common keywords into an `Option` containing a `DateExpr::InNDays(i32)`.
pub fn parse_keywords(text: &str) -> Option<DateExpression> {
    // today, tomorrow, yesterday

    let re = Regex::new(r"(?i)\b(?P<key>today|tomorrow|yesterday)\b").unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(key_match) = caps.name("key") {
            let n = match key_match.as_str().to_lowercase().as_ref() {
                "today" => 0,
                "tomorrow" => 1,
                "yesterday" => -1,
                _ => 0,
            };
            return Some(DateExpression::InXDays(n));
        }
    }

    None
}

#[cfg(test)]
mod parse_keywords_works_when {
    use crate::language::{
        en::expressions::parse_keywords::parse_keywords, shared::DateExpression,
    };

    #[test]
    fn day_keywords() {
        assert_in_n_days("tomorrow", 1);
        assert_in_n_days("yesterday", -1);
        assert_in_n_days("today", 0);
    }

    fn assert_in_n_days(text: &str, n: i32) {
        assert_eq!(parse_keywords(text), Some(DateExpression::InXDays(n)))
    }
}
