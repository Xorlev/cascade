use crate::slogd_proto::LogEntry;
use crate::storage::segment::Segment;
use crate::storage::{Log, LogQuery, Offset, StorageError, StorageResult, TopicName};
use async_trait::async_trait;

/// Topic is the underlying structure which manages multiple LogSegments. As each segment
/// becomes too large or has been open for too long, the Topic closes that segment and
/// opens a new segment. Topic implements the Log methods as it must decide which segment(s)
/// are necessary to query to answer a given request.
pub struct Topic {
    // TODO, make this many segments
    segment: Segment,
}

impl Topic {
    pub fn create(topic_name: TopicName) -> Topic {
        Topic {
            segment: Segment {},
        }
    }
}

#[async_trait]
impl Log for Topic {
    async fn append(&mut self, entries: Vec<LogEntry>) -> StorageResult<Offset> {
        unimplemented!()
    }

    async fn read(&self, query: LogQuery) -> StorageResult<Vec<LogEntry>> {
        self.segment.read(query).await
    }

    async fn flush(&mut self) {
        unimplemented!()
    }

    fn close(&mut self) {
        unimplemented!()
    }
}
