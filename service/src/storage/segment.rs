use crate::storage::{Log, StorageResult, Offset, LogQuery};
use futures::Stream;
use chrono::NaiveDateTime;

/// LogSegment both manages the individual log files and their indices. For each append operation,
/// LogSegment indexes the new messages (if necessary, not all index operations are required to
/// index all messages) and
struct LogSegment {

}

#[async_trait]
impl Log for LogSegment {
    async fn append(entries: Vec<LogEntry>) -> StorageResult<Offset> {
        unimplemented!()
    }

    async fn read(query: LogQuery) -> Box<dyn Stream<Item = StorageResult<LogEntry>>> {
        unimplemented!()
    }

    fn close() {
        unimplemented!()
    }
}

impl LogSegment {
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