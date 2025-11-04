use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use crate::{error::BigFileError, reader::BigFileReader};

#[derive(Clone, Copy)]
pub(crate) struct Entry {
    pub offset: u64,
    pub size: u64,
}

pub(crate) struct Bfdb {
    pub entries: HashMap<u64, Entry>,
}

impl Bfdb {
    pub(crate) fn from(reader: &mut BigFileReader<impl Read + Seek>) -> Result<Self, BigFileError> {
        let len = reader.read_u32_le()?;
        let mut entries = HashMap::with_capacity(len as _);

        for _ in 0..len {
            let size = reader.read_u64_le()?;
            let offset = reader.read_u64_le()?;
            let hash = reader.read_u64_le()?;

            entries.insert(hash, Entry { offset, size });
        }

        Ok(Bfdb { entries })
    }
}
