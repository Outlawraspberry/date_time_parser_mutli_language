use chrono::Weekday;
use regex::Regex;

use crate::language::shared::DateExpression;

pub fn parse_day_in_relative_week(text: &str) -> Option<DateExpression> {
    let re = Regex::new(r"(?i)((?P<day>mon|tue|wed|thu|fri|sat|sun)(r?day|r?sday|nesday|urday)?\s(?P<prep>last|this|next)\sweek\b|(?P<prep2>last|this|next)\sweek\s(on\s)?(?P<day2>mon|tue|wed|thu|fri|sat|sun)(r?day|r?sday|nesday|urday)?\b)")
        .unwrap();

    if let Some(caps) = re.captures(text) {
        if let Some(prep_match) = caps.name("prep") {
            let relative_week = match prep_match.as_str().to_lowercase().as_ref() {
                "next" => 1,
                "last" => -1,
                "this" => 0,
                _ => 0,
            };

            if let Some(caps) = re.captures(text) {
                if let Some(day_match) = caps.name("day") {
                    let d = day_match
                        .as_str()
                        .to_lowercase()
                        .parse::<Weekday>()
                        .unwrap();
                    return Some(DateExpression::DayInXWeeks(relative_week, d));
                }
            }
        }
    }

    if let Some(caps) = re.captures(text) {
        if let Some(prep_match) = caps.name("prep2") {
            let relative_week = match prep_match.as_str().to_lowercase().as_ref() {
                "next" => 1,
                "last" => -1,
                "this" => 0,
                _ => 0,
            };

            if let Some(caps) = re.captures(text) {
                if let Some(day_match) = caps.name("day2") {
                    let d = day_match
                        .as_str()
                        .to_lowercase()
                        .parse::<Weekday>()
                        .unwrap();
                    return Some(DateExpression::DayInXWeeks(relative_week, d));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod parse_day_in_relative_week_works_when {
    use chrono::Weekday;

    use super::parse_day_in_relative_week;
    use crate::language::shared::DateExpression;

    use super::super::super::en_date_parser::weekday_to_english;

    #[test]
    fn relative_week_day_is_parsed() {
        let mut cases: Vec<(String, i32, Weekday)> = Vec::new();

        for weekday in get_weekday_vector() {
            let input = format!("next week {}", weekday);
            cases.push((input, 1, weekday));

            let input = format!("next week {}", weekday_to_english(&weekday));
            cases.push((input, 1, weekday));

            let input = format!("next week on {}", weekday);
            cases.push((input, 1, weekday));

            let input = format!("next week on {}", weekday_to_english(&weekday));
            cases.push((input, 1, weekday));

            let input = format!("this week {}", weekday);
            cases.push((input, 0, weekday));

            let input = format!("this week {}", weekday_to_english(&weekday));
            cases.push((input, 0, weekday));

            let input = format!("this week on {}", weekday);
            cases.push((input, 0, weekday));

            let input = format!("this week on {}", weekday_to_english(&weekday));
            cases.push((input, 0, weekday));

            let input = format!("last week {}", weekday);
            cases.push((input, -1, weekday));

            let input = format!("last week {}", weekday_to_english(&weekday));
            cases.push((input, -1, weekday));

            let input = format!("last week on {}", weekday);
            cases.push((input, -1, weekday));

            let input = format!("last week on {}", weekday_to_english(&weekday));
            cases.push((input, -1, weekday));
        }

        for (input, in_weeks, weekday) in cases {
            assert_in_n_days(input.as_str(), in_weeks, weekday);
        }
    }

    #[test]
    fn day_in_relative_week_is_parsed() {
        let mut cases: Vec<(String, i32, Weekday)> = Vec::new();

        for weekday in get_weekday_vector() {
            let input = format!("{} next week", weekday);
            cases.push((input, 1, weekday));

            let input = format!("{} next week", weekday_to_english(&weekday));
            cases.push((input, 1, weekday));

            let input = format!("{} this week", weekday);
            cases.push((input, 0, weekday));

            let input = format!("{} this week", weekday_to_english(&weekday));
            cases.push((input, 0, weekday));

            let input = format!("{} last week", weekday);
            cases.push((input, -1, weekday));

            let input = format!("{} last week", weekday_to_english(&weekday));
            cases.push((input, -1, weekday));
        }

        for (input, in_weeks, weekday) in cases {
            assert_in_n_days(input.as_str(), in_weeks, weekday);
        }
    }

    fn assert_in_n_days(text: &str, in_weeks: i32, weekday: Weekday) {
        assert_eq!(
            parse_day_in_relative_week(text),
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
