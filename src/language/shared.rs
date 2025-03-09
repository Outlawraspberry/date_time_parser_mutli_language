#[derive(PartialEq, Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(PartialEq, Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

pub fn num_to_month(num: u32) -> Option<Month> {
    match num {
        1 => Some(Month::January),
        2 => Some(Month::February),
        3 => Some(Month::March),
        4 => Some(Month::April),
        5 => Some(Month::May),
        6 => Some(Month::June),
        7 => Some(Month::July),
        8 => Some(Month::August),
        9 => Some(Month::September),
        10 => Some(Month::October),
        11 => Some(Month::November),
        12 => Some(Month::December),
        _ => None,
    }
}

#[cfg(test)]
mod shared_components_work_when {
    use crate::language::shared::*;

    #[test]
    fn some_number_is_parsed_to_month() {
        assert_eq!(num_to_month(1), Some(Month::January));
        assert_eq!(num_to_month(2), Some(Month::February));
        assert_eq!(num_to_month(3), Some(Month::March));
        assert_eq!(num_to_month(4), Some(Month::April));
        assert_eq!(num_to_month(5), Some(Month::May));
        assert_eq!(num_to_month(6), Some(Month::June));
        assert_eq!(num_to_month(7), Some(Month::July));
        assert_eq!(num_to_month(8), Some(Month::August));
        assert_eq!(num_to_month(9), Some(Month::September));
        assert_eq!(num_to_month(10), Some(Month::October));
        assert_eq!(num_to_month(11), Some(Month::November));
        assert_eq!(num_to_month(12), Some(Month::December));
        assert_eq!(num_to_month(123), None);
    }
}
