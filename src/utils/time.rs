use chrono::{FixedOffset, Utc};

pub fn generate_time() -> String {
    let now_utc = Utc::now();
    let vietnam_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    now_utc.with_timezone(&vietnam_offset).to_rfc2822()
}

pub fn generate_unix_timestamp() -> i64 {
    let now_utc = Utc::now();
    let vietnam_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    now_utc.with_timezone(&vietnam_offset).timestamp()
}
