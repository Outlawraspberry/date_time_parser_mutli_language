use date_time_parser_multi_language::{DateFormat, DateParser, EnDateParser};

fn main() {
    let some_input = "Remind me that I have to check my mails tomorrow.";

    let now = chrono::Utc::now().naive_local().date();

    let date =
        EnDateParser::search_relative_date_expression(some_input, &now, &DateFormat::DayMonthYear);

    println!("I found the date {:?}", date);
}
