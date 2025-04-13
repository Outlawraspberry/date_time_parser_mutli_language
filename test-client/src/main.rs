use std::io::stdin;
use date_time_parser_multi_language::{DateFormat, DateParser, EnDateParser, StartDayOfWeek};

fn main() {

    println!("hello to the date_time_parser_mutliple_langauge test interface. :3\n");
    
    what_todo_next();
}

fn what_todo_next() {
    let mut action = String::new();

    println!("What do you want? 1 = Test the parser, 2 = exit");
    stdin().read_line(&mut action).unwrap();

    let action = action.trim();

    if action == "1" {
        parser_date();
    } else if action == "2" {
        println!("Thanks for using, see you later! :)\n");

        std::process::exit(2);
    } else {
        what_todo_next();
    }
}

fn parser_date() {
    let mut message = String::new();

    println!("Please type in your mesage with the date information:");
    stdin().read_line(&mut message).unwrap();

    let now = chrono::Utc::now().naive_local().date();
    let date = EnDateParser::search_relative_date_expression(&message, &now, &DateFormat::DayMonthYear, &StartDayOfWeek::Monday);

    match date {
        Some(date) => {
            println!("\nFound {}\n", date);
        }
        None => println!("\nNo date information found in the provided message!\n"),
    }

    what_todo_next();
}
