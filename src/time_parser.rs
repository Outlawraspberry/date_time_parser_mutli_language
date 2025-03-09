use chrono::NaiveTime;

pub trait TimeParser {
    fn search_time(input: &str) -> &NaiveTime;
}
