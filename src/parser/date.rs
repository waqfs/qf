use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc, offset::LocalResult};
use chrono_tz::Tz;

pub fn parse_date(input: &str, default_tz: &str) -> Result<DateTime<Utc>, String> {
    let input = input.trim();

    if let Ok(result) = DateTime::parse_from_rfc3339(input) {
        return Ok(result.with_timezone(&Utc));
    }

    let timezone: Tz = default_tz
        .parse()
        .map_err(|e| format!("An error occurred parsing the default timezone: {e}"))?;

    if let Ok(local) = NaiveDate::parse_from_str(input, "%d/%m/%Y") {
        let local = local
            .and_hms_opt(0, 0, 0)
            .expect("An error occurred supplying 00:00:00 to date.");
        return tz_to_utc(local, timezone);
    }

    if let Ok(local) = NaiveDateTime::parse_from_str(input, "%d/%m/%Y %H:%M:%S%.f") {
        return tz_to_utc(local, timezone);
    }

    if let Ok(local) = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S%.f") {
        return tz_to_utc(local, timezone);
    }

    Err("Invalid date format.".to_string())
}

fn tz_to_utc(local: NaiveDateTime, timezone: Tz) -> Result<DateTime<Utc>, String> {
    match timezone.from_local_datetime(&local) {
        LocalResult::Single(result) => Ok(result.with_timezone(&Utc)),
        LocalResult::Ambiguous(first, second) => Err("Default timezone is ambiguous.".to_string()),
        LocalResult::None => Err("Default timezone is invalid.".to_string()),
    }
}
