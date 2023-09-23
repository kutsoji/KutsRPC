use chrono::{
    Duration,
    NaiveDateTime,
    NaiveTime,
    TimeZone,
    Timelike,
    Utc,
};
use leptos::RwSignal;

pub fn get_timestamp(time: RwSignal<Option<String>>) -> Option<String> {
    todo!()
}

pub fn to_unix(time: String) -> i64 {
    let parsed_time = NaiveTime::parse_from_str(&time, "%H:%M:%S").unwrap();

    let now = Utc::now();
    let today = now.date_naive();

    let datetime = today.and_time(parsed_time);

    datetime.timestamp()
}

pub fn format_hms_from_unix(unix_timestamp: String) -> String {
    let datetime = NaiveDateTime::from_timestamp_opt(
        unix_timestamp.parse().expect("Error parsing the timestamp string"),
        0,
    )
    .expect("Error getting NaiveDateTime");

    let time = datetime.time();
    format!("{:02}:{:02}:{:02}", time.hour(), time.minute(), time.second())
}
