use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use sqlx::types::time::{OffsetDateTime, PrimitiveDateTime, Time as Tm,};

pub struct DateTimeBridge;

impl DateTimeBridge {
    pub fn to_string(date_time: DateTime<Utc>) -> String {
        date_time.to_rfc3339()
    }

    pub fn from_string(date_time: String) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(&date_time).unwrap()
    }

    pub fn from_offset_date_time(date_time: OffsetDateTime) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(date_time.unix_timestamp())
            .unwrap()
    }

    pub fn from_date_time(date_time: DateTime<Utc>) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(date_time.timestamp())
        .unwrap()
            
    }

    pub fn from_primitive_to_date_time(date_time: DateTime<Utc>) -> PrimitiveDateTime{
        PrimitiveDateTime::new(DateTimeBridge::from_date_time(date_time).date(), DateTimeBridge::from_date_time(date_time).time())
    }
}
