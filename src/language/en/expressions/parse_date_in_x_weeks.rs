use regex::Regex;

use crate::language::{en::en_date_parser::string_to_num_english, shared::DateExpression};

/// Parses a `str` into an `Option` containing a `DateExpression::InWeek(i8, Weekday)`
pub fn parse_in_x_weeks(text: &str) -> Option<DateExpression> {
    // sat, this saturday, next saturday, last saturday, this sat,

    let re = Regex::new(r"(in\s(?P<num>([\d]{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\s(weeks?))").unwrap();

    if let Some(caps) = re.captures(&String::from(text).trim().to_lowercase()) {
        if let Some(num_match) = caps.name("num") {
            if let Ok(num) = num_match.as_str().parse::<i32>() {
                return Some(DateExpression::InXWeeks(num));
            } else {
                let num_str = num_match.as_str();
                let num = string_to_num_english(num_str).unwrap();
                return Some(DateExpression::InXWeeks(num));
            }
        }
    }

    None
}

#[cfg(test)]
mod parse_relative_date_works_when {
    use crate::language::shared::DateExpression;

    use super::parse_in_x_weeks;

    #[test]
    fn expression_with_digit_is_found() {
        assert_in_n_days("Lunch in 6 weeks", 6);
        assert_in_n_days("Lunch in 1 week", 1);
        assert_in_n_days("Lunch in 300 weeks", 300);
    }

    #[test]
    fn test_parse_date_expression() {
        // Test cases for textual representations of numbers
        let cases = vec![
            ("in one week", Some(DateExpression::InXWeeks(1))),
            ("in One week", Some(DateExpression::InXWeeks(1))),
            ("in ONE weeks", Some(DateExpression::InXWeeks(1))),
            ("in two weeks", Some(DateExpression::InXWeeks(2))),
            ("in Two weeks", Some(DateExpression::InXWeeks(2))),
            ("in TWO weeks", Some(DateExpression::InXWeeks(2))),
            ("in three weeks", Some(DateExpression::InXWeeks(3))),
            ("in four weeks", Some(DateExpression::InXWeeks(4))),
            ("in five weeks", Some(DateExpression::InXWeeks(5))),
            ("in six weeks", Some(DateExpression::InXWeeks(6))),
            ("in seven weeks", Some(DateExpression::InXWeeks(7))),
            ("in eight weeks", Some(DateExpression::InXWeeks(8))),
            ("in nine weeks", Some(DateExpression::InXWeeks(9))),
            ("in ten weeks", Some(DateExpression::InXWeeks(10))),
            ("in eleven weeks", Some(DateExpression::InXWeeks(11))),
            ("in twelve weeks", Some(DateExpression::InXWeeks(12))),
        ];

        for (input, expected) in cases {
            let result = parse_in_x_weeks(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_invalid_cases() {
        // Test cases for invalid inputs
        let invalid_cases = vec![
            "in zero weeks",
            "in thirteen weeks",
            "in weeks",
            "in one",
            "in twelve",
            "in twelve apples",
        ];

        for input in invalid_cases {
            let result = parse_in_x_weeks(input);
            assert_eq!(result, None, "Expected None for input: {}", input);
        }
    }

    fn assert_in_n_days(text: &str, n: i32) {
        assert_eq!(parse_in_x_weeks(text), Some(DateExpression::InXWeeks(n)))
    }
}
