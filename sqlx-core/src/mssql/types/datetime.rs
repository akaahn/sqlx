use std::io::Write;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::mssql::protocol::type_info::{DataType, TypeInfo};
use crate::mssql::{Mssql, MssqlTypeInfo, MssqlValueRef};

use crate::types::Type;
use chrono::{
    DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, Offset, TimeZone, Utc,
};
use crate::database::{HasArguments, HasValueRef};

impl Type<Mssql> for DateTime<Utc> {
    fn type_info() -> MssqlTypeInfo {
        MssqlTypeInfo(TypeInfo::new(DataType::DateTime, 8))
    }
}

impl Encode<'_, Mssql> for DateTime<Utc> {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        Encode::<Mssql>::encode(self.naive_utc(), buf)
    }
}

impl<'r> Decode<'r, Mssql> for DateTime<Utc> {
    fn decode(value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let naive = <NaiveDateTime as Decode<Mssql>>::decode(value)?;
        Ok(Utc.from_utc_datetime(&naive))
    }
}

impl Type<Mssql> for DateTime<Local> {
    fn type_info() -> MssqlTypeInfo {
        MssqlTypeInfo(TypeInfo::new(DataType::DateTime, 8))
    }
}

impl Encode<'_, Mssql> for DateTime<Local> {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        Encode::<Mssql>::encode(self.naive_utc(), buf)
    }
}

impl<'r> Decode<'r, Mssql> for DateTime<Local> {
    fn decode(value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let local = <DateTime<Utc> as Decode<Mssql>>::decode(value)?.with_timezone(&Local);
        Ok(local)
    }
}

impl Type<Mssql> for NaiveDateTime {
    fn type_info() -> MssqlTypeInfo {
        MssqlTypeInfo(TypeInfo::new(DataType::DateTime, 8))
    }
}

impl Encode<'_, Mssql> for NaiveDateTime {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        let epoch = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        let us = (*self - epoch)
            .num_microseconds()
            .unwrap_or_else(|| panic!("NaiveDateTime out of range for SQL Server: {:?}", self));

        Encode::<Mssql>::encode(&us, buf)
    }
}

impl<'r> Decode<'r, Mssql> for NaiveDateTime {
    fn decode(value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let buf = value.as_bytes()?;
        let epoch = NaiveDate::from_ymd(1900, 1, 1)
            .and_hms(0, 0, 0);

        let days_since = &buf[..4];
        let days_since = LittleEndian::read_i32(days_since) as i64;

        let ticks = &buf[4..8];
        let ticks = ticks_to_ms(LittleEndian::read_u32(ticks));

        Ok(epoch + Duration::days(days_since) + Duration::milliseconds(ticks))
    }
}

/// Convert ticks to ms according to https://stackoverflow.com/a/1143266
/// Vague, but returns correct result
fn ticks_to_ms(ticks: u32) -> i64 {
    (ticks as f64 * 3.33333333333333333333) as i64
}
