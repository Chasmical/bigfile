use crate::{error::Result, reader::BigFileReader};
use std::{
    io::{Read, Seek},
    path::PathBuf,
};

pub(crate) struct Bfn {
    pub(crate) files: Vec<PathBuf>,
}

impl Bfn {
    pub(crate) fn from(reader: &mut BigFileReader<impl Read + Seek>) -> Result<Self> {
        let mut files = Vec::new();

        fn read_dir(
            reader: &mut BigFileReader<impl Read + Seek>,
            parent: &PathBuf,
            out: &mut Vec<PathBuf>,
        ) -> Result<()> {
            let name_len = reader.read_u32_le()?;
            let name = reader.read_string(name_len as _)?;
            let mut cur_path = parent.clone();
            cur_path.push(name);

            let file_count = reader.read_u32_le()?;
            for _ in 0..file_count {
                let len = reader.read_u32_le()?;
                let file_name = reader.read_string(len as _)?;
                let mut file_path = cur_path.clone();
                file_path.push(file_name);
                out.push(file_path);
            }

            let subdir_count = reader.read_u32_le()?;
            for _ in 0..subdir_count {
                read_dir(reader, &cur_path, out)?;
            }
            Ok(())
        }

        let root = PathBuf::new();
        read_dir(reader, &root, &mut files)?;

        Ok(Bfn { files })
    }
}
