use regex::Regex;

use crate::language::{en::en_date_parser::string_to_num_english, shared::DateExpression};

/// Parses a `str` into an `Option` containing a `DateExpression::InWeek(i8, Weekday)`
pub fn parse_in_x_weeks(text: &str) -> Option<DateExpression> {
    // sat, this saturday, next saturday, last saturday, this sat,

    let re = Regex::new(r"(in\s(?P<num>([\d]{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\s(weeks?))").unwrap();

    if let Some(caps) = re.captures(&String::from(text).trim().to_lowercase()) {
        if let Some(num_match) = caps.name("num") {
            if let Ok(num) = num_match.as_str().parse::<i32>() {
                return Some(DateExpression::DayInXWeeks(num, chrono::Weekday::Sun));
            } else {
                let num_str = num_match.as_str();
                let num = string_to_num_english(num_str).unwrap();
                return Some(DateExpression::DayInXWeeks(num, chrono::Weekday::Sun));
            }
        }
    }

    None
}

#[cfg(test)]
mod parse_relative_date_works_when {
    use crate::language::shared::DateExpression;
    use chrono::Weekday;

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
            (
                "in one week",
                Some(DateExpression::DayInXWeeks(1, Weekday::Sun)),
            ),
            (
                "in One week",
                Some(DateExpression::DayInXWeeks(1, Weekday::Sun)),
            ),
            (
                "in ONE weeks",
                Some(DateExpression::DayInXWeeks(1, Weekday::Sun)),
            ),
            (
                "in two weeks",
                Some(DateExpression::DayInXWeeks(2, Weekday::Sun)),
            ),
            (
                "in Two weeks",
                Some(DateExpression::DayInXWeeks(2, Weekday::Sun)),
            ),
            (
                "in TWO weeks",
                Some(DateExpression::DayInXWeeks(2, Weekday::Sun)),
            ),
            (
                "in three weeks",
                Some(DateExpression::DayInXWeeks(3, Weekday::Sun)),
            ),
            (
                "in four weeks",
                Some(DateExpression::DayInXWeeks(4, Weekday::Sun)),
            ),
            (
                "in five weeks",
                Some(DateExpression::DayInXWeeks(5, Weekday::Sun)),
            ),
            (
                "in six weeks",
                Some(DateExpression::DayInXWeeks(6, Weekday::Sun)),
            ),
            (
                "in seven weeks",
                Some(DateExpression::DayInXWeeks(7, Weekday::Sun)),
            ),
            (
                "in eight weeks",
                Some(DateExpression::DayInXWeeks(8, Weekday::Sun)),
            ),
            (
                "in nine weeks",
                Some(DateExpression::DayInXWeeks(9, Weekday::Sun)),
            ),
            (
                "in ten weeks",
                Some(DateExpression::DayInXWeeks(10, Weekday::Sun)),
            ),
            (
                "in eleven weeks",
                Some(DateExpression::DayInXWeeks(11, Weekday::Sun)),
            ),
            (
                "in twelve weeks",
                Some(DateExpression::DayInXWeeks(12, Weekday::Sun)),
            ),
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
        assert_eq!(
            parse_in_x_weeks(text),
            Some(DateExpression::DayInXWeeks(n, Weekday::Sun))
        )
    }
}
