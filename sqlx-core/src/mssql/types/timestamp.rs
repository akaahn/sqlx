use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::mssql::protocol::type_info::{DataType, TypeInfo};
use crate::mssql::{Mssql, MssqlTypeInfo, MssqlValueRef};
use crate::types::Type;

impl Type<Mssql> for [u8; 8] {
    fn type_info() -> MssqlTypeInfo {
        MssqlTypeInfo(TypeInfo::new(DataType::BigBinary, 8))
    }
}

impl Encode<'_, Mssql> for [u8; 8] {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        buf.extend(self);

        IsNull::No
    }
}

impl<'r> Decode<'r, Mssql> for [u8; 8] {
    fn decode(value: MssqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let bytes = value.as_bytes()?;
        assert_eq!(bytes.len(), 8);

        let mut result: [u8; 8] = Default::default();
        result.copy_from_slice(&bytes[..8]);

        Ok(result)
    }
}
