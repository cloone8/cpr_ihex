use anyhow::Result;
use std::{
    fmt::Display,
    io::{BufRead, Lines},
};

use crate::{to_u16_be, to_u32_be};

use super::{
    raw::{parse_ihex, RawIHexRecord},
    DataRecord, ExtendedLinearAddressRecord, ExtendedSegmentAddressRecord, IHexRecord,
    StartLinearAddressRecord, StartSegmentAddressRecord,
};

macro_rules! expect_length {
    ($nu:expr, $len:expr) => {
        if $nu.len() != $len {
            return Err(InvalidIHexRecordError::InvalidDataSizeForType);
        }
    };
}

struct BaseAddrs {
    segment: Option<ExtendedSegmentAddressRecord>,
    linear: Option<ExtendedLinearAddressRecord>,
}

#[derive(Debug, Clone, Copy)]
pub struct SegmentStartAddr {
    pub code_segment: u16,
    pub instruction_pointer: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum StartAddr {
    Segment(SegmentStartAddr),
    Linear(u32),
}

#[derive(Debug)]
pub struct IHexFile {
    pub records: Vec<IHexRecord>,
    filetype: IHexFileType,
    start_address: Option<StartAddr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IHexFileType {
    IHex8 = 8,
    IHex16 = 16,
    IHex32 = 32,
}

impl Display for IHexFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IHexFileType::IHex8 => write!(f, "8-bit"),
            IHexFileType::IHex16 => write!(f, "16-bit"),
            IHexFileType::IHex32 => write!(f, "32-bit"),
        }
    }
}

impl IHexFile {
    pub fn filetype(&self) -> IHexFileType {
        self.filetype
    }

    pub fn start_address(&self) -> Option<StartAddr> {
        self.start_address
    }

    fn determine_filetype(records: &Vec<IHexRecord>) -> IHexFileType {
        let mut filetype = IHexFileType::IHex8;

        for record in records {
            match record {
                IHexRecord::ExtendedSegmentAddress(_) => {
                    filetype = filetype.max(IHexFileType::IHex16)
                }
                IHexRecord::StartSegmentAddress(_) => filetype = filetype.max(IHexFileType::IHex16),
                IHexRecord::ExtendedLinearAddress(_) => {
                    filetype = filetype.max(IHexFileType::IHex32)
                }
                IHexRecord::StartLinearAddress(_) => filetype = filetype.max(IHexFileType::IHex32),
                _ => {}
            }
        }

        filetype
    }

    pub fn read<T: BufRead>(lines: Lines<T>) -> Result<Self> {
        let mut raw_records = Vec::new();

        for line in lines {
            let line = line?;
            let record = parse_ihex(&line)?;
            raw_records.push(record);
        }

        let mut bases = BaseAddrs {
            segment: None,
            linear: None,
        };

        let mut records = Vec::with_capacity(raw_records.len());
        let mut start_addr: Option<StartAddr> = None;

        for raw_record in raw_records {
            Self::parse_and_append(&mut records, raw_record, &mut bases, &mut start_addr)?;
        }

        let ihex_file = IHexFile {
            filetype: Self::determine_filetype(&records),
            records,
            start_address: start_addr,
        };

        Ok(ihex_file)
    }

    pub fn data_bytes(&self) -> Vec<u8> {
        let records: Vec<_> = self
            .records
            .iter()
            .filter_map(|record| match record {
                IHexRecord::Data(data_record) => Some(data_record),
                _ => None,
            })
            .collect();

        let mut data = Vec::new();

        for record in records {
            let effective_addr = record.calc_effective_address();

            data.resize(effective_addr as usize + record.data.len(), 0);
            for (i, byte) in record.data.iter().enumerate() {
                data[effective_addr as usize + i] = *byte;
            }
        }

        data
    }

    fn parse_and_append(
        records: &mut Vec<IHexRecord>,
        value: RawIHexRecord,
        bases: &mut BaseAddrs,
        start_addr: &mut Option<StartAddr>,
    ) -> Result<(), InvalidIHexRecordError> {
        if !value.checksum_valid() {
            return Err(InvalidIHexRecordError::Checksum);
        }

        let rec = match value.rectyp {
            0 => IHexRecord::Data(DataRecord {
                segment_address: bases.segment.clone(),
                linear_address: bases.linear.clone(),
                naive_address: value.load_offset,
                data: value.data,
            }),
            1 => IHexRecord::EndOfFile,
            2 => {
                expect_length!(value.data, 2);
                let segment_base = to_u16_be!(value.data);
                IHexRecord::ExtendedSegmentAddress(ExtendedSegmentAddressRecord {
                    segment_base: (segment_base as usize) << 4,
                })
            }
            3 => {
                expect_length!(value.data, 4);
                let code_segment = to_u16_be!(value.data[0..2].to_vec());
                let instruction_pointer = to_u16_be!(value.data[2..].to_vec());

                match start_addr {
                    Some(prev) => log::warn!("Multiple start addresses found in IHex file, overwriting previous start address ({:?})", prev),
                    None => {
                        *start_addr = Some(StartAddr::Segment(SegmentStartAddr {
                            code_segment,
                            instruction_pointer,
                        }));
                    }
                }

                IHexRecord::StartSegmentAddress(StartSegmentAddressRecord {
                    code_segment,
                    instruction_pointer,
                })
            }
            4 => {
                expect_length!(value.data, 2);
                let address_base = to_u16_be!(value.data);
                IHexRecord::ExtendedLinearAddress(ExtendedLinearAddressRecord { address_base })
            }
            5 => {
                expect_length!(value.data, 4);

                let entry_point = to_u32_be!(value.data);

                match start_addr {
                    Some(prev) => log::warn!("Multiple start addresses found in IHex file, overwriting previous start address ({:?})", prev),
                    None => {
                        *start_addr = Some(StartAddr::Linear(entry_point));
                    }
                }

                IHexRecord::StartLinearAddress(StartLinearAddressRecord { entry_point })
            }
            _ => return Err(InvalidIHexRecordError::RecordType),
        };

        match rec {
            IHexRecord::ExtendedSegmentAddress(ref addr) => {
                bases.segment = Some(addr.clone());
            }
            IHexRecord::ExtendedLinearAddress(ref addr) => {
                bases.linear = Some(addr.clone());
            }
            _ => {}
        }

        records.push(rec);

        Ok(())
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

impl std::error::Error for InvalidIHexRecordError {}
