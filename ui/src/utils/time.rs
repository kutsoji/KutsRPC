use chrono::{
    Local,
    NaiveDateTime,
    NaiveTime,
    Timelike,
    Utc,
};

use crate::invoke;

pub async fn get_timestamp(time: Option<String>) -> Result<Option<String>, String> {
    match time {
        Some(ref s) if s == "SinceDiscordStarted" => {
            match invoke!("get_process_start_time", {name: String = "Discord".to_string()}, String)
            {
                Ok(t) => Ok(Some(t)),
                Err(_) => Err("No Discord Instance Detected.\nLaunch Discord and Try Again.".to_string()),
            }
        }
        Some(ref s) if s == "SinceKutsRpcStarted" => {
            match invoke!("get_process_start_time", {name: String = "KutsRPC".to_string()}, String)
            {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err("No KutsRPC Instance Detected.\nMake Sure You Are Running The Desktop App Rather Than The Web App.".to_string()),
            }
        }
        Some(ref s) if s == "SinceLastActivity" => Ok(Some(current_timestamp().to_string())),
        Some(ref s) if s == "LocalTime" => match Local::now().date().and_hms_opt(0, 0, 0) {
            Some(t) => Ok(Some(t.with_timezone(&Utc).timestamp().to_string())),
            None => Ok(None),
        },
        Some(ref s) if s == "Custom" => Ok(None),
        Some(s) => Ok(Some(to_unix_from_hms(s).unwrap().to_string())),
        None => Ok(None),
    }
}

pub fn get_time(current: i64, timestamp: String) -> String {
    let i64_timestamp = timestamp.parse::<i64>().expect("couldn't parse timestamp.");
    let mut result: String = String::new();
    if current >= i64_timestamp {
        let diff: i64 = current - i64_timestamp;

        let datetime =
            NaiveDateTime::from_timestamp_opt(diff, 0).expect("Error getting NaiveDateTime");

        let time = datetime.time();
        result = format!("{:02}:{:02}:{:02} elapsed", time.hour(), time.minute(), time.second());
    } else {
        let diff: i64 = i64_timestamp - current;

        let datetime =
            NaiveDateTime::from_timestamp_opt(diff, 0).expect("Error getting NaiveDateTime");

        let time = datetime.time();
        result = format!("{:02}:{:02}:{:02} left", time.hour(), time.minute(), time.second());
    }
    result
}

pub fn to_unix_from_hms(time: String) -> Result<i64, String> {
    let parsed_time = match NaiveTime::parse_from_str(&time, "%H:%M:%S") {
        Ok(time) => time,
        Err(_) => return Err("Invalid time format. Use hh:mm:ss".to_string()),
    };
    let local_today = Local::now().date();
    let local_datetime = local_today.and_time(parsed_time);
    if let Some(local_datetime) = local_datetime {
        let utc_datetime = local_datetime.with_timezone(&Utc);
        Ok(utc_datetime.timestamp())
    } else {
        Err("Invalid combination of date and time.".to_string())
    }
}

pub fn current_timestamp() -> i64 {
    Utc::now().timestamp()
}
