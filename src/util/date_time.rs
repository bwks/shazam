use chrono::{DateTime, Datelike, Local, NaiveDate};

/// Current date as string
pub fn date_today() -> String {
    let dt: DateTime<Local> = Local::now();
    dt.format("%Y-%m-%d").to_string()
}

/// Get a human readable date from a date string
/// date_string must be of the format 2022-10-30 (yyyy-mm-dd)
/// Example: converts 2022-10-30 -> 30th of October 2022
pub fn human_date(date_string: String) -> String {
    let date = NaiveDate::parse_from_str(date_string.as_str(), "%Y-%m-%d")
        .unwrap_or(NaiveDate::from_ymd(1979, 01, 01));
    let day_suffix = match date.day() {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 24 | 25
        | 26 | 27 | 28 | 29 | 30 => "th",
        _ => "",
    };
    format!(
        "{}{} of {} {}",
        date.day(),
        day_suffix,
        date.format("%B"),
        date.year()
    )
}
