use chrono::prelude::*;

pub fn convert_utc_to_date(utc: DateTime<Utc>, show_timezone: bool) -> String {
    // let utc = Utc::now();
    // let local = Local::now();
    let converted: DateTime<Local> = DateTime::from(utc);
    if show_timezone {
        converted.format("%m/%d/%Y %Z").to_string()
    } else {
        converted.format("%m/%d/%Y").to_string()
    }
}

pub fn convert_utc_to_datetime(utc: DateTime<Utc>, show_timezone: bool) -> String {
    // let utc = Utc::now();
    // let local = Local::now();
    let converted: DateTime<Local> = DateTime::from(utc);
    if show_timezone {
        converted.format("%m/%d/%Y %I:%M %p %Z").to_string()
    } else {
        converted.format("%m/%d/%Y %I:%M %p").to_string()
    }
}

pub fn convert_utc_to_datetime_with_seconds(utc: DateTime<Utc>, show_timezone: bool) -> String {
    // let utc = Utc::now();
    // let local = Local::now();
    let converted: DateTime<Local> = DateTime::from(utc);
    if show_timezone {
        converted.format("%m/%d/%Y %I:%M:%S %p %Z").to_string()
    } else {
        converted.format("%m/%d/%Y %I:%M:%S %p").to_string()
    }
}
