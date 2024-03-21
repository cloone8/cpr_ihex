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
}
