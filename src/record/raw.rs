use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::to_u16_be;

#[derive(Debug)]
pub struct RawIHexRecord {
    pub reclen: u8,
    pub load_offset: u16,
    pub rectyp: u8,
    pub data: Vec<u8>,
    pub checksum: u8,
}

fn add_u16(a: u8, b: u16) -> u8 {
    let u16_bytes = u16::to_ne_bytes(b);

    a.wrapping_add(u16_bytes[0]).wrapping_add(u16_bytes[1])
}

impl RawIHexRecord {
    pub fn generate_checksum(&self) -> u8 {
        let non_data_sum = add_u16(self.reclen.wrapping_add(self.rectyp), self.load_offset);
        let data_sum = self
            .data
            .iter()
            .fold(non_data_sum, |sum, byte| sum.wrapping_add(*byte));

        data_sum.wrapping_neg()
    }
    pub fn checksum_valid(&self) -> bool {
        self.generate_checksum() == self.checksum
    }
}

impl From<hex::FromHexError> for IHexParseError {
    fn from(_: hex::FromHexError) -> Self {
        IHexParseError::NonHexString
    }
}

pub fn parse_ihex(value: &str) -> Result<RawIHexRecord, IHexParseError> {
    if !value.is_ascii() {
        return Err(IHexParseError::NonAsciiString);
    }

    if !value.starts_with(':') {
        return Err(IHexParseError::MissingStartCode);
    }

    let mut bytes = hex::decode(&value[1..])?;
    let checksum = bytes.pop().ok_or(IHexParseError::RecordTooShort)?;

    bytes.reverse(); // reverse to make it easier to pop from the end

    let byte_count = bytes.pop().ok_or(IHexParseError::RecordTooShort)?;
    let addr_bytes = pop_n(&mut bytes, 2)?;
    let address = to_u16_be!(addr_bytes);
    let record_type = bytes.pop().ok_or(IHexParseError::RecordTooShort)?;

    if bytes.len() != byte_count as usize {
        return Err(IHexParseError::IncorrectDataSize);
    }

    bytes.reverse(); // reverse back to original order

    Ok(RawIHexRecord {
        reclen: byte_count,
        load_offset: address,
        rectyp: record_type,
        data: bytes,
        checksum,
    })
}

#[derive(Debug)]
pub enum IHexParseError {
    MissingStartCode,
    NonAsciiString,
    RecordTooShort,
    IncorrectDataSize,
    NonHexString,
}

impl Display for IHexParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IHexParseError::MissingStartCode => write!(f, "Record is missing start code"),
            IHexParseError::NonAsciiString => write!(f, "Record contains non-ASCII characters"),
            IHexParseError::RecordTooShort => write!(
                f,
                "Reached end of input before a complete record was parsed"
            ),
            IHexParseError::IncorrectDataSize => {
                write!(f, "Data size does not match byte count in record")
            }
            IHexParseError::NonHexString => write!(f, "Record contains non-hex characters"),
        }
    }
}

impl Error for IHexParseError {}

fn pop_n<T>(vec: &mut Vec<T>, n: usize) -> Result<Vec<T>, IHexParseError> {
    let mut result = Vec::new();

    for _ in 0..n {
        result.push(vec.pop().ok_or(IHexParseError::RecordTooShort)?);
    }

    Ok(result)
}
