use std::{
    fs::File,
    io::{self, BufReader, Error, ErrorKind, Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::error::{BigFileError, IoResultExt};

pub(crate) struct BigFileReader<R: Read + Seek> {
    inner: R,
    file: Option<PathBuf>,
}

impl<R: Read + Seek> BigFileReader<R> {
    pub(crate) fn new(reader: R) -> Self {
        BigFileReader {
            inner: reader,
            file: None,
        }
    }

    fn pos(&mut self) -> Option<usize> {
        if let Ok(pos) = self.inner.stream_position() {
            Some(pos as _)
        } else {
            None
        }
    }

    pub(crate) fn read_u32_le(&mut self) -> Result<u32, BigFileError> {
        let pos = self.pos();
        let mut buf = [0; 4];
        self.read_exact(&mut buf)
            .with_offset(self.file.clone(), pos)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub(crate) fn read_u64_le(&mut self) -> Result<u64, BigFileError> {
        let pos = self.pos();
        let mut buf = [0; 8];
        self.read_exact(&mut buf)
            .with_offset(self.file.clone(), pos)?;
        Ok(u64::from_le_bytes(buf))
    }

    pub(crate) fn read_string(&mut self, len: usize) -> Result<String, BigFileError> {
        let pos = self.pos();
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)
            .with_offset(self.file.clone(), pos)?;

        if let Ok(string) = String::from_utf8(buf) {
            Ok(string)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "read string was not UTF-8",
            ))
            .with_offset(self.file.clone(), pos)
        }
    }
}

impl BigFileReader<BufReader<File>> {
    pub(crate) fn from_path(path: PathBuf) -> Result<Self, BigFileError> {
        let inner = File::open(&path).with_file(path.clone())?;
        Ok(Self {
            inner: BufReader::new(inner),
            file: Some(path),
        })
    }
}

impl<R: Read + Seek> Read for BigFileReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: Read + Seek> Seek for BigFileReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.inner.seek(pos)
    }
}
