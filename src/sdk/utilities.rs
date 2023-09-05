use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use sqlx::types::time::OffsetDateTime;

pub struct DateTimeBridge;

impl DateTimeBridge {
    pub fn to_string(date_time: DateTime<Utc>) -> String {
        date_time.to_rfc3339()
    }

    pub fn from_string(date_time: String) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(&date_time).unwrap()
    }

    pub fn from_offset_date_time(offset_date_time: OffsetDateTime) -> DateTime<Utc> {
        let naive_date_time =
            NaiveDateTime::from_timestamp_millis(offset_date_time.unix_timestamp() * 1000).unwrap();

        // TimeZone::from_utc_datetime(&Utc, &naive_date_time)
        Utc.from_utc_datetime(&naive_date_time)
        // DateTime::<Utc>::from_utc(naive_date_time, Utc)
    }

    pub fn from_date_time(date_time: DateTime<Utc>) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(date_time.timestamp()).unwrap()
    }
}
