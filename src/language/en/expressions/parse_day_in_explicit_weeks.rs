use chrono::Weekday;
use regex::Regex;

use crate::language::{en::en_date_parser::string_to_num_english, shared::DateExpression};

pub fn parse_day_in_explicit_week(text: &str) -> Option<DateExpression> {
    let re = Regex::new(r"(?i)((?P<day>mon|tue|wed|thu|fri|sat|sun)(r?day|r?sday|nesday|urday)?\s(in\s)?((?P<num>([\d]{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\s(weeks?))\b|(in\s(?P<num2>([\d]{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\s(weeks?))\s(on\s)?(?P<day2>mon|tue|wed|thu|fri|sat|sun)(r?day|r?sday|nesday|urday)?\b)")
        .unwrap();

    if let Some(caps) = re.captures(&String::from(text).trim().to_lowercase()) {
        if let Some(num_match) = caps.name("num") {
            let in_weeks = if let Ok(num) = num_match.as_str().parse::<i32>() {
                num
            } else {
                let num_str = num_match.as_str();
                string_to_num_english(num_str).unwrap()
            };

            if let Some(caps) = re.captures(text) {
                if let Some(day_match) = caps.name("day") {
                    let d = day_match
                        .as_str()
                        .to_lowercase()
                        .parse::<Weekday>()
                        .unwrap();
                    return Some(DateExpression::DayInXWeeks(in_weeks, d));
                }
            }
        }
    }
    if let Some(caps) = re.captures(&String::from(text).trim().to_lowercase()) {
        if let Some(num_match) = caps.name("num2") {
            let in_weeks = if let Ok(num) = num_match.as_str().parse::<i32>() {
                num
            } else {
                let num_str = num_match.as_str();
                string_to_num_english(num_str).unwrap()
            };

            if let Some(caps) = re.captures(text) {
                if let Some(day_match) = caps.name("day2") {
                    let d = day_match
                        .as_str()
                        .to_lowercase()
                        .parse::<Weekday>()
                        .unwrap();
                    return Some(DateExpression::DayInXWeeks(in_weeks, d));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod parse_day_in_relative_week_works_when {
    use chrono::Weekday;

    use super::parse_day_in_explicit_week;
    use crate::language::shared::DateExpression;

    use super::super::super::en_date_parser::weekday_to_english;

    #[test]
    fn relative_week_day_is_parsed() {
        let mut cases: Vec<(String, i32, Weekday)> = Vec::new();

        for weekday in get_weekday_vector() {
            let input = format!("in three weeks {}", weekday);
            cases.push((input, 3, weekday));

            let input = format!("in three weeks {}", weekday_to_english(&weekday));
            cases.push((input, 3, weekday));

            let input = format!("in three weeks on {}", weekday);
            cases.push((input, 3, weekday));

            let input = format!("in three weeks on {}", weekday_to_english(&weekday));
            cases.push((input, 3, weekday));
        }

        for (input, in_weeks, weekday) in cases {
            assert_in_n_days(input.as_str(), in_weeks, weekday);
        }
    }

    #[test]
    fn day_in_relative_week_is_parsed() {
        let mut cases: Vec<(String, i32, Weekday)> = Vec::new();

        for weekday in get_weekday_vector() {
            let input = format!("{} in three weeks", weekday);
            cases.push((input, 3, weekday));

            let input = format!("{} in three weeks", weekday_to_english(&weekday));
            cases.push((input, 3, weekday));

            let input = format!("{} in 3 weeks", weekday_to_english(&weekday));
            cases.push((input, 3, weekday));

            let input = format!("{} in 3 weeks", weekday_to_english(&weekday));
            cases.push((input, 3, weekday));
        }

        for (input, in_weeks, weekday) in cases {
            assert_in_n_days(input.as_str(), in_weeks, weekday);
        }
    }

    fn assert_in_n_days(text: &str, in_weeks: i32, weekday: Weekday) {
        assert_eq!(
            parse_day_in_explicit_week(text),
            Some(DateExpression::DayInXWeeks(in_weeks, weekday)),
            "Failed to parse {:?} from {}",
            Some(DateExpression::DayInXWeeks(in_weeks, weekday)),
            text
        )
    }

    fn get_weekday_vector() -> Vec<Weekday> {
        vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ]
    }
}
