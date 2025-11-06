mod bfdb;
mod bfn;
pub mod error;
mod reader;

use std::{
    collections::HashMap,
    fs,
    io::{Cursor, Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::{
    bfdb::Bfdb,
    bfn::Bfn,
    error::{BigFileError, IoResultExt},
    reader::BigFileReader,
};

pub struct Entry {
    offset: u64,
    size: u64,
}

pub enum DataSource {
    File(PathBuf),
    Buffer(Cursor<Vec<u8>>),
}

pub struct BigFile {
    entries: HashMap<PathBuf, Entry>,
    bfdata: DataSource,
}

impl BigFile {
    pub fn entries(&self) -> &HashMap<PathBuf, Entry> {
        &self.entries
    }

    pub fn from_paths(
        bfn_path: PathBuf,
        bfdb_path: PathBuf,
        bfdata_path: PathBuf,
    ) -> Result<Self, BigFileError> {
        let mut reader = BigFileReader::from_path(bfn_path)?;
        let bfn = Bfn::from(&mut reader)?;

        let mut reader = BigFileReader::from_path(bfdb_path)?;
        let bfdb = Bfdb::from(&mut reader)?;

        BigFile::from(bfn, bfdb, DataSource::File(bfdata_path))
    }

    fn from(bfn: Bfn, bfdb: Bfdb, bfdata: DataSource) -> Result<Self, BigFileError> {
        let mut entries = HashMap::with_capacity(bfn.files.len());
        for path in bfn.files {
            let hash = fnv1a(&path.to_str().unwrap().replace('\\', "/").to_lowercase()[2..]);

            let entry = bfdb.entries[&hash];
            entries.insert(
                path,
                Entry {
                    offset: entry.offset,
                    size: entry.size,
                },
            );
        }

        Ok(BigFile { entries, bfdata })
    }

    pub fn new<R: Read + Seek>(
        bfn_reader: &mut R,
        bfdb_reader: &mut R,
        bfdata_reader: &mut R,
    ) -> Result<Self, BigFileError> {
        let mut bfn = BigFileReader::new(bfn_reader);
        let mut bfdb = BigFileReader::new(bfdb_reader);

        let mut buf = Vec::new();
        bfdata_reader.read_to_end(&mut buf)?;
        let cursor = Cursor::new(buf);

        BigFile::from(
            Bfn::from(&mut bfn)?,
            Bfdb::from(&mut bfdb)?,
            DataSource::Buffer(cursor),
        )
    }

    pub fn get(&self, file: &PathBuf) -> Result<Vec<u8>, BigFileError> {
        let entry = match self.entries.get(file) {
            Some(v) => v,
            None => return Err(BigFileError::EntryNotFound(file.clone())),
        };

        let mut data = vec![0; entry.size as _];

        match &self.bfdata {
            DataSource::File(path_buf) => {
                let mut reader = BigFileReader::from_path(path_buf.clone())?;

                reader.seek(SeekFrom::Start(entry.offset))?;
                reader.read_exact(&mut data)?;
            }
            DataSource::Buffer(cursor) => {
                let mut reader = BigFileReader::new(cursor.clone());

                reader.seek(SeekFrom::Start(entry.offset))?;
                reader.read_exact(&mut data)?;
            }
        };

        Ok(data)
    }

    pub fn extract(&self, output_path: PathBuf) -> Result<(), BigFileError> {
        match &self.bfdata {
            DataSource::File(path_buf) => {
                let mut reader = BigFileReader::from_path(path_buf.clone())?;
                return self.extract_inner(output_path, &mut reader);
            }
            DataSource::Buffer(cursor) => {
                let mut reader = BigFileReader::new(cursor.clone());
                return self.extract_inner(output_path, &mut reader);
            }
        };
    }

    fn extract_inner(
        &self,
        output_path: PathBuf,
        reader: &mut BigFileReader<impl Read + Seek>,
    ) -> Result<(), BigFileError> {
        for (path, entry) in &self.entries {
            let mut data = vec![0; entry.size as _];

            reader.seek(SeekFrom::Start(entry.offset))?;
            reader.read_exact(&mut data)?;

            let path = std::env::current_dir()
                .expect("Failed to get current directory")
                .join(&output_path)
                .join(&path);

            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, data).with_file(path)?;
        }
        Ok(())
    }
}

fn fnv1a(string: &str) -> u64 {
    let mut hash: u64 = 0xCBF29CE484222325;
    for char in string.chars() {
        hash ^= char as u64;
        hash = hash.wrapping_mul(0x100000001B3);
    }
    hash
}
