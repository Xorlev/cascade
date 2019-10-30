use async_trait::async_trait;
use failure::Fail;
use futures::{Future, Stream};

use crate::slogd_proto::LogEntry;
use std::collections::HashMap;

mod segment;

#[derive(Debug, Fail)]
pub enum StorageError {
    #[fail(display = "An unknown error occurred.")]
    UnknownError,
}

type Offset = u64;
type StorageResult<T> = Result<T, StorageError>;
type TopicName = String;


#[async_trait]
trait Log {
    async fn append(entries: Vec<LogEntry>) -> StorageResult<Offset>;
    async fn read(query: LogQuery) -> Box<dyn Stream<Item = StorageResult<LogEntry>>>;
    fn close();
}

/// LogManager maintains topics, including running periodic maintenance tasks.
struct TopicManager {
    topics: HashMap<TopicName, Topic>
}

impl TopicManager {
    pub fn new() -> TopicManager {
        let mut topics = HashMap::new();
        // Load topics from disk.

        TopicManager {
            topics
        }
    }
}

/// LogQuery is a Rust representation of a log query. A log query has three primary parts:
///
/// 1) A starting offset. This can be either a special offset (earliest, latest), an actual u64
///    offset, or some kind of index query (such as by timestamp).
///
/// 2) A filter. Currently this is just an annotation post-filter.
///
/// 3) Options. This includes behavior such as a maximum number of messages to return.
struct LogQuery;


/// Topic is the underlying structure which manages multiple LogSegments. As each segment
/// becomes too large or has been open for too long, the Topic closes that segment and
/// opens a new segment. Topic implements the Log methods as it must decide which segment(s)
/// are necessary to query to answer a given request.
struct Topic {

}

#[async_trait]
impl Log for Topic {
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

struct IndexQuery;
struct IndexResponse;

/// An index could index over the set of annotations available on each LogEntry,
/// (or a specific set of annotations) and provide a mapping from annotation to a list of
/// matching entry ranges. Entry ranges can then be intersected to find the appropriate segments
/// to read and stream over starting from oldest to newest.
#[async_trait]
trait Index {
    async fn lookup(query: IndexQuery) -> StorageResult<IndexResponse>;
    async fn index(entry: LogEntry) -> StorageResult<()>;
}