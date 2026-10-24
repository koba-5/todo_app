use time::{OffsetDateTime, UtcOffset};

pub(crate) fn now() -> OffsetDateTime {
    let utc_datetime = OffsetDateTime::now_utc();
    const JST: UtcOffset = match UtcOffset::from_hms(9, 0, 0) {
        Ok(v) => v,
        Err(_) => unreachable!(),
    };
    utc_datetime.to_offset(JST)
}
