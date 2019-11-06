use crate::slogd_proto::LogEntry;
use crate::storage::{LogQuery, Offset, StorageError, StorageResult};
use async_trait::async_trait;

enum IndexResponse {
    NotFound,
    FilePosition(usize),

    // These types require additional index lookups to continue resolving.
    // TODO: This can probably be implemented as a looping state machine which always seeks to find
    //  one of the terminal states (NotFound or FilePosition). You can imagine multiple index types
    //  such as timestamp or indexed annotation returning Offset and needing the second-level
    //  resolution to a file position.
    Offset(Offset),
}

/// An index could index over the set of annotations available on each LogEntry,
/// (or a specific set of annotations) and provide a mapping from annotation to a list of
/// matching entry ranges. Entry ranges can then be intersected to find the appropriate segments
/// to read and stream over starting from oldest to newest.
#[async_trait]
trait Index {
    async fn lookup(&self, query: LogQuery) -> StorageResult<IndexResponse>;
    async fn index(&mut self, entry: LogEntry) -> StorageResult<()>;
}

/// DumbIndex blackholes requests to index a new value and lookups always return position=0, forcing
/// a full scan of the segment.
struct DumbIndex;

#[async_trait]
impl Index for DumbIndex {
    async fn lookup(&self, query: LogQuery) -> Result<IndexResponse, StorageError> {
        // With th
        Ok(IndexResponse::FilePosition(0))
    }

    async fn index(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        Ok(())
    }
}
