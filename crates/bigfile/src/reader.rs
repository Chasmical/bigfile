use std::{
    fs::File,
    io::{self, BufReader, Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::error::{IoErrorExt, IoResultExt, Result};

pub(crate) struct BigFileReader<R: Read + Seek> {
    inner: R,
    file: Option<PathBuf>,
}

impl<R: Read + Seek> BigFileReader<R> {
    pub(crate) fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let offset = match pos {
            SeekFrom::Start(v) => v,
            SeekFrom::End(v) => v as _,
            SeekFrom::Current(v) => v as _,
        };

        self.inner
            .seek(pos)
            .with_offset(self.file.clone(), Some(offset as _))
    }

    pub(crate) fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let offset = self.pos();
        self.inner
            .read_exact(buf)
            .with_offset(self.file.clone(), offset)
    }

    pub(crate) fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let offset = self.pos();
        self.inner
            .read_to_end(buf)
            .with_offset(self.file.clone(), offset)
    }

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

    pub(crate) fn read_u32_le(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub(crate) fn read_u64_le(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    pub(crate) fn read_string(&mut self, len: usize) -> Result<String> {
        let pos = self.pos();
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;

        if let Ok(string) = String::from_utf8(buf) {
            Ok(string)
        } else {
            Err(
                io::Error::new(io::ErrorKind::InvalidData, "read string was not UTF-8")
                    .with_offset(self.file.clone(), pos),
            )
        }
    }
}

impl BigFileReader<BufReader<File>> {
    pub(crate) fn from_path(path: PathBuf) -> Result<Self> {
        let inner = File::open(&path).with_file(path.clone())?;
        Ok(Self {
            inner: BufReader::new(inner),
            file: Some(path),
        })
    }
}
