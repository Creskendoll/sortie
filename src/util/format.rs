use chrono::{DateTime, Local};

pub fn format_date(date: &DateTime<Local>) -> String {
    let date = date.format("%Y-%m-%d").to_string();
    return date;
}
