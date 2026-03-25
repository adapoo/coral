use chrono::{DateTime, Utc};


pub fn format_ratio(value: f64) -> String {
    let s = format!("{:.2}", value);
    s.strip_suffix(".00").map(String::from).unwrap_or(s)
}


pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}


pub fn format_timestamp(ts: i64) -> String {
    let ts_millis = if ts > 10_000_000_000 { ts } else { ts * 1000 };
    DateTime::<Utc>::from_timestamp_millis(ts_millis)
        .map(|dt| dt.format("%m/%d/%y").to_string())
        .unwrap_or_else(|| "N/A".to_string())
}
