use anyhow::Result as AnyhowResult;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use std::collections::HashMap;
use tera::{to_value, try_get_value, Result, Value};

/// Current date as string
pub fn date_today() -> String {
    let dt: DateTime<Local> = Local::now();
    dt.format("%Y-%m-%d").to_string()
}

/// Parse the a date string in the (yyyy-mm-dd) into a naive date.
pub fn to_date(date_string: String) -> AnyhowResult<NaiveDate> {
    let date = NaiveDate::parse_from_str(date_string.as_str(), "%Y-%m-%d")
        .unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1));
    Ok(date)
}

/// Get a human readable date from a date string
/// date_string must be of the format 2022-10-30 (yyyy-mm-dd)
/// Example: converts 2022-10-30 -> 30th of October 2022
pub fn human_date(date_string: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("human_date", "date_string", String, date_string);
    let date = NaiveDate::parse_from_str(s.as_str(), "%Y-%m-%d")
        .unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1));
    let day_suffix = match date.day() {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 24 | 25
        | 26 | 27 | 28 | 29 | 30 => "th",
        _ => "",
    };
    let result = format!(
        "{}{} of {} {}",
        date.day(),
        day_suffix,
        date.format("%B"),
        date.year()
    );
    Ok(to_value(result)?)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::util::date_time::to_date;

    // #[test]
    // fn returns_date_as_humananize_string() {
    //     let test_cases = vec![
    //         ("2022-10-01".to_owned(), "1st of October 2022".to_owned()),
    //         ("2022-10-22".to_owned(), "22nd of October 2022".to_owned()),
    //         ("2022-10-03".to_owned(), "3rd of October 2022".to_owned()),
    //         ("2022-10-30".to_owned(), "30th of October 2022".to_owned()),
    //     ];
    //     for t in test_cases {
    //         let result = human_date(t.0);
    //         assert_eq!(result, t.1);
    //     }
    // }

    #[test]
    fn returns_naieve_date_from_date_string() {
        let test_cases = vec![
            (
                "2022-10-01".to_owned(),
                NaiveDate::parse_from_str("2022-10-01", "%Y-%m-%d").unwrap(),
            ),
            (
                "".to_owned(),
                NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap(),
            ),
        ];
        for t in test_cases {
            let result = to_date(t.0).unwrap();
            assert_eq!(result, t.1);
        }
    }
}
