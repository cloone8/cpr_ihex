pub mod file;
pub mod raw;

use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataRecord {
    segment_address: Option<ExtendedSegmentAddressRecord>,
    linear_address: Option<ExtendedLinearAddressRecord>,
    pub naive_address: u16,
    pub data: Vec<u8>,
}

impl Hash for DataRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.naive_address.hash(state);
        self.data.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedSegmentAddressRecord {
    pub segment_base: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartSegmentAddressRecord {
    pub code_segment: u16,
    pub instruction_pointer: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedLinearAddressRecord {
    pub address_base: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartLinearAddressRecord {
    pub entry_point: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IHexRecord {
    Data(DataRecord),
    EndOfFile,
    ExtendedSegmentAddress(ExtendedSegmentAddressRecord),
    StartSegmentAddress(StartSegmentAddressRecord),
    ExtendedLinearAddress(ExtendedLinearAddressRecord),
    StartLinearAddress(StartLinearAddressRecord),
}
