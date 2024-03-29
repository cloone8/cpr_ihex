pub mod file;
pub mod raw;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataRecord {
    segment_address: Option<ExtendedSegmentAddressRecord>,
    linear_address: Option<ExtendedLinearAddressRecord>,
    pub naive_address: u16,
    pub data: Vec<u8>,
}

impl DataRecord {
    pub fn calc_effective_address(&self) -> u32 {
        let linear_base: u32 = match &self.linear_address {
            Some(linear) => (linear.address_base as u32) << 16,
            None => 0,
        };

        let segment_base: u32 = match &self.segment_address {
            Some(segment) => segment.segment_base as u32,
            None => 0,
        };

        linear_base + segment_base + (self.naive_address as u32)
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
