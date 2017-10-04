extern crate chrono;

use chrono::*;
use chrono::offset::utc::UTC;
use chrono::Duration;

/// Calculates the `DateTime<UTC>` value for 1 Gigasecond (10^9 s) after the provided `start_date`.
pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    start_date + Duration::seconds(1_000_000_000i64)
}
