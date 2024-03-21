use std::{error::Error, fmt::Display};

use crate::{raw_ihex_record::RawIHexRecord, to_u16_be, to_u32_be};

#[derive(Debug)]
pub struct DataRecord {
    pub naive_address: u16,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ExtendedSegmentAddressRecord {
    pub segment_base: usize,
}
#[derive(Debug)]
pub struct StartSegmentAddressRecord {
    pub code_segment: u16,
    pub instruction_pointer: u16,
}
#[derive(Debug)]
pub struct ExtendedLinearAddressRecord {
    pub address_base: u16,
}
#[derive(Debug)]
pub struct StartLinearAddressRecord {
    pub entry_point: u32,
}

#[derive(Debug)]
pub enum IHexRecord {
    Data(DataRecord),
    EndOfFile,
    ExtendedSegmentAddress(ExtendedSegmentAddressRecord),
    StartSegmentAddress(StartSegmentAddressRecord),
    ExtendedLinearAddress(ExtendedLinearAddressRecord),
    StartLinearAddress(StartLinearAddressRecord),
}

macro_rules! expect_length {
    ($nu:expr, $len:expr) => {
        if $nu.len() != $len {
            return Err(InvalidIHexRecordError::InvalidDataSizeForType);
        }
    };
}

impl TryFrom<RawIHexRecord> for IHexRecord {
    type Error = InvalidIHexRecordError;

    fn try_from(value: RawIHexRecord) -> Result<Self, Self::Error> {
        if !value.checksum_valid() {
            return Err(InvalidIHexRecordError::Checksum);
        }

        match value.record_type {
            0 => Ok(IHexRecord::Data(DataRecord {
                naive_address: value.address,
                data: value.data,
            })),
            1 => Ok(IHexRecord::EndOfFile),
            2 => {
                expect_length!(value.data, 2);
                let segment_base = to_u16_be!(value.data);
                Ok(IHexRecord::ExtendedSegmentAddress(
                    ExtendedSegmentAddressRecord {
                        segment_base: (segment_base as usize) * 16,
                    },
                ))
            }
            3 => {
                expect_length!(value.data, 4);
                let code_segment = to_u16_be!(value.data[0..2].to_vec());
                let instruction_pointer = to_u16_be!(value.data[2..].to_vec());
                Ok(IHexRecord::StartSegmentAddress(StartSegmentAddressRecord {
                    code_segment,
                    instruction_pointer,
                }))
            }
            4 => {
                expect_length!(value.data, 2);
                let address_base = to_u16_be!(value.data);
                Ok(IHexRecord::ExtendedLinearAddress(
                    ExtendedLinearAddressRecord { address_base },
                ))
            }
            5 => {
                expect_length!(value.data, 4);

                let entry_point = to_u32_be!(value.data);
                Ok(IHexRecord::StartLinearAddress(StartLinearAddressRecord {
                    entry_point,
                }))
            }
            _ => Err(InvalidIHexRecordError::RecordType),
        }
    }
}

#[derive(Debug)]
pub enum InvalidIHexRecordError {
    Checksum,
    RecordType,
    InvalidDataSizeForType,
}

impl Display for InvalidIHexRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidIHexRecordError::Checksum => write!(f, "Invalid checksum"),
            InvalidIHexRecordError::RecordType => write!(f, "Invalid record type"),
            InvalidIHexRecordError::InvalidDataSizeForType => {
                write!(f, "Invalid data size for record type")
            }
        }
    }
}

impl Error for InvalidIHexRecordError {}
