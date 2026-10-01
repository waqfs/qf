use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

pub fn parse_date(input: &str, default_tz: &str) -> Result<DateTime<Utc>, String> {
    let input = input.trim();

    if let Ok(result) = DateTime::parse_from_rfc3339(input) {
        return Ok(result.with_timezone(&Utc));
    }

    Err("Invalid date format.".to_string())
}
