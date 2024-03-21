use std::{
    fs::{self, File},
    io::{self, stdout, BufRead, BufReader, Write},
};

#[derive(Debug)]
struct RawIHexRecord {
    byte_count: u8,
    address: u16,
    record_type: u8,
    data: Vec<u8>,
    checksum: u8,
}

#[derive(Debug)]
struct DataRecord {
    pub naive_address: u16,
    pub data: Vec<u8>,
}

#[derive(Debug)]
struct ExtendedSegmentAddressRecord {
    pub segment_base: usize,
}
#[derive(Debug)]
struct StartSegmentAddressRecord {
    pub code_segment: u16,
    pub instruction_pointer: u16,
}
#[derive(Debug)]
struct ExtendedLinearAddressRecord {
    pub address_base: u16,
}
#[derive(Debug)]
struct StartLinearAddressRecord {
    pub entry_point: u32,
}

#[derive(Debug)]
enum IHexRecord {
    Data(DataRecord),
    EndOfFile,
    ExtendedSegmentAddress(ExtendedSegmentAddressRecord),
    StartSegmentAddress(StartSegmentAddressRecord),
    ExtendedLinearAddress(ExtendedLinearAddressRecord),
    StartLinearAddress(StartLinearAddressRecord),
}

#[derive(Debug)]
enum InvalidIHexRecordError {
    InvalidChecksum,
    InvalidRecordType,
    InvalidDataSizeForType,
}

macro_rules! to_u16_be {
    ($x:expr) => {{
        if $x.len() != 2 {
            panic!("Invalid byte length");
        }

        u16::from_be_bytes([$x[0], $x[1]])
    }};
}

macro_rules! to_u32_be {
    ($x:expr) => {{
        if $x.len() != 4 {
            panic!("Invalid byte length");
        }

        u32::from_be_bytes([$x[0], $x[1], $x[2], $x[3]])
    }};
}

fn add_u16(a: u8, b: u16) -> u8 {
    let u16_bytes = u16::to_ne_bytes(b);

    a.wrapping_add(u16_bytes[0]).wrapping_add(u16_bytes[1])
}

impl RawIHexRecord {
    pub fn generate_checksum(&self) -> u8 {
        let non_data_sum = add_u16(self.byte_count.wrapping_add(self.record_type), self.address);
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
            return Err(InvalidIHexRecordError::InvalidChecksum);
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
            _ => Err(InvalidIHexRecordError::InvalidRecordType),
        }
    }
}

#[derive(Debug)]
enum IHexParseError {
    MissingStartCode,
    NonAsciiString,
    RecordTooShort,
    IncorrectDataSize,
    NonHexString,
}

impl From<hex::FromHexError> for IHexParseError {
    fn from(_: hex::FromHexError) -> Self {
        IHexParseError::NonHexString
    }
}

fn remove_first_char(s: &str) -> &str {
    &s[1..]
}

fn pop_n<T>(vec: &mut Vec<T>, n: usize) -> Result<Vec<T>, IHexParseError> {
    let mut result = Vec::new();

    for _ in 0..n {
        result.push(vec.pop().ok_or(IHexParseError::RecordTooShort)?);
    }

    Ok(result)
}

fn parse_ihex(value: &str) -> Result<RawIHexRecord, IHexParseError> {
    if !value.is_ascii() {
        return Err(IHexParseError::NonAsciiString);
    }

    if !value.starts_with(':') {
        return Err(IHexParseError::MissingStartCode);
    }

    let mut bytes = hex::decode(remove_first_char(value))?;
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
        byte_count,
        address,
        record_type,
        data: bytes,
        checksum,
    })
}

fn main() {
    let file = File::open("C:\\Users\\Woute\\Downloads\\Caterina.hex").unwrap();
    let reader = BufReader::new(file);

    let raw_records: Vec<RawIHexRecord> = reader
        .lines()
        .map(|line| line.map(|l| parse_ihex(l.as_str()).unwrap()))
        .map(|record| record.unwrap())
        .collect();

    raw_records.iter().for_each(|record| {
        println!("{:?}", record);
    });
    
    let records = raw_records
        .into_iter()
        .map(|record| IHexRecord::try_from(record).unwrap())
        .collect::<Vec<IHexRecord>>();

    records.iter().for_each(|record| {
        match record {
            IHexRecord::Data(data) => {
                println!("DataRecord - addr: {:x} data {:?}", data.naive_address, data.data.iter().map(|x| *x as char).collect::<Vec<char>>());
            }
            _ => println!("{:?}", record)
        }
    });

    println!("{}", records.len());

    let datarecords: Vec<&DataRecord> = records.iter()
        .filter_map(|record| {
            if let IHexRecord::Data(data) = record {
                Some(data)
            } else {
                None
            }
        })
        .collect();

    let mut fake_ram: Vec<u8> = Vec::new();

    for record in datarecords {
        let start = record.naive_address as usize;
        let end = start + record.data.len();

        if fake_ram.len() < end {
            fake_ram.resize(end, 0);
        }

        fake_ram[start..end].copy_from_slice(&record.data);
    }
    File::create("test_bin.bin").unwrap().write_all(&fake_ram).unwrap();
    // write!(, )
    // println!("{:?}", fake_ram);
}
