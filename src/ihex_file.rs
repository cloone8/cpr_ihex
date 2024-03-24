use anyhow::{anyhow, Error, Result};
use std::io::{BufRead, Lines};

use crate::{ihex_record::IHexRecord, raw_ihex_record::parse_ihex};

#[derive(Debug)]
pub struct IHexFile {
    pub records: Vec<IHexRecord>,
}

impl IHexFile {
    pub fn read<T: BufRead>(lines: Lines<T>) -> Result<Self> {
        let mut raw_records = Vec::new();

        for line in lines {
            let line = line?;
            let record = parse_ihex(&line)?;
            raw_records.push(record);
        }

        let records: Vec<IHexRecord> = raw_records
            .into_iter()
            .map(IHexRecord::try_from)
            .map(|result| result.map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<IHexRecord>, Error>>()?;

        Ok(IHexFile { records })
    }

    pub fn data_bytes(&self) -> Vec<u8> {
        if self.records.iter().any(|record| matches!(record, IHexRecord::ExtendedLinearAddress(_))) {
            panic!("Extended linear address records are not supported yet");
        }

        if self.records.iter().any(|record| matches!(record, IHexRecord::ExtendedSegmentAddress(_))) {
            panic!("Extended segment address records are not supported yet");
        }

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
            data.resize(record.naive_address as usize + record.data.len(), 0);
            for (i, byte) in record.data.iter().enumerate() {
                data[record.naive_address as usize + i] = *byte;
            }
        }

        data
    }
}
