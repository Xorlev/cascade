use chrono::NaiveDateTime;
use futures::Stream;

use async_trait::async_trait;

use crate::slogd_proto::LogEntry;
use crate::storage::{Log, LogQuery, Offset, StorageError, StorageResult};

/// LogSegment both manages the individual log files and their indices. For each append operation,
/// LogSegment indexes the new messages (if necessary, not all index operations are required to
/// index all messages) and writes the new message to the end of the log.
pub struct Segment {}

#[async_trait]
impl Log for Segment {
    async fn append(&mut self, entries: Vec<LogEntry>) -> Result<u64, StorageError> {
        unimplemented!()
    }

    async fn read(&self, query: LogQuery) -> StorageResult<Vec<LogEntry>> {
        let mut logs = Vec::new();
        logs.push(LogEntry {
            offset: 0,
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
