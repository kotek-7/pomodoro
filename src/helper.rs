use std::time::Duration;

pub fn format_duration(duration: Duration) -> String {
    let total_sec = duration.as_secs();
    let sub_min = total_sec / 60;
    let sub_sec = total_sec % 60;
    format!("{0: >02}:{1: >02}", sub_min, sub_sec)
}
