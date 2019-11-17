use chrono::NaiveDateTime;

use async_std::path::{Path, PathBuf};
use async_trait::async_trait;

use crate::slogd_proto::LogEntry;
use crate::storage::{Log, LogQuery, Offset, StorageError, StorageResult};
use async_std::prelude::*;
use tokio::io::Error;
use std::io;
use async_std::task::spawn_blocking;
use std::fs::File;
use std::os::unix::fs::FileExt;
use async_std::sync::{Arc};
use std::sync::RwLock;
use std::io::BufReader;
use crate::storage::file::LogFile;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;



/// LogSegment both manages the individual log files and their indices. For each append operation,
/// LogSegment indexes the new messages (if necessary, not all index operations are required to
/// index all messages) and writes the new message to the end of the log.
pub struct Segment {
    log: LogFile
}

impl Segment {
    pub async fn open<P: AsRef<Path>> (base_path: P, start_offset: Offset) -> StorageResult<Segment> {
        let mut path = PathBuf::new();
        path.push(base_path);
        path.push(format!("{}.log", start_offset));
        println!("Opening segment: {:?}", path.as_path());
        let log = LogFile::open(path).await?;

        Ok(Segment {
            log
        })
    }
}

#[async_trait]
impl Log for Segment {
    async fn append(&mut self, entries: Vec<LogEntry>) -> StorageResult<u64> {
//        self.f.write(&[]).await?


        unimplemented!()
    }

    async fn read(&self, query: LogQuery) -> StorageResult<Vec<LogEntry>> {
        let mut buf = vec![0; 8];
        self.log.read_at(&mut buf, 0).await?;
        let mut cursor = Cursor::new(buf);


        println!("Next msg size: {:?}", cursor.read_u64::<BigEndian>().unwrap());
//        println!("Read {} bytes: {:?}", read_bytes, contents);

        let mut logs = Vec::new();
        logs.push(LogEntry {
            offset: 123,
            timestamp: None,
            annotations: Default::default(),
            entry: None,
        });

        Ok(logs)
    }

    async fn flush(&mut self) {
        unimplemented!()
    }

    fn close(&mut self) {
        unimplemented!()
    }
}

impl Segment {
    /**
    StartOffset() uint64
    EndOffset() uint64
    StartTime() time.Time
    EndTime() time.Time
    SizeBytes() uint64
    Delete() error
    Flush() error
    Close() error
    */

    pub fn start_offset(&self) -> Offset {
        unimplemented!()
    }

    pub fn end_offset(&self) -> Offset {
        unimplemented!()
    }

    pub fn start_timestamp(&self) -> NaiveDateTime {
        unimplemented!()
    }

    pub fn end_timestamp(&self) -> NaiveDateTime {
        unimplemented!()
    }

    pub fn size_bytes(&self) -> usize {
        unimplemented!()
    }

    pub fn length(&self) -> usize {
        (self.end_offset() - self.start_offset()) as usize
    }
}

impl From<std::io::Error> for StorageError {
    fn from(err: Error) -> Self {
        StorageError::IoError(err)
    }
}