extern crate failure;
#[macro_use] extern crate failure_derive;
extern crate futures;
extern crate slogd_shared;

use futures::{Future, Stream};
use slogd_shared::protos;
use slogd_shared::protos::data::LogEntry;

#[derive(Debug, Fail)]
pub enum StorageError {
    #[fail(display = "An unknown error occurred.")]
    UnknownError,
}

struct LogQuery;

trait Log {
    fn append(entries: Vec<LogEntry>) -> Box<dyn Future<Item = i32, Error = StorageError>>;
    fn read(query: LogQuery) -> Box<dyn Stream<Item = LogEntry, Error = StorageError>>;
    fn close();
}

/// LogManager is the underlying structure which manages multiple LogSegments. As each segment
/// becomes too large or has been open for too long, the LogManager closes that segment and
/// opens a new segment. LogManager implements the Log methods as it must decide which segment(s)
/// are necessary to query to answer a given request.
struct LogManager {

}

/// LogSegment both manages the individual log files and their indices. For each append operation,
/// LogSegment indexes the new messages (if necessary, not all index operations are required to
/// index all messages) and
struct LogSegment {

}

impl Log for LogSegment {
    fn append(entries: Vec<LogEntry>) -> Box<dyn Future<Item=i32, Error=StorageError>> {
        unimplemented!()
    }

    fn read(query: LogQuery) -> Box<dyn Stream<Item=LogEntry, Error=StorageError>> {
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
trait Index {
    fn lookup(query: IndexQuery) -> Box<dyn Future<Item = IndexResponse, Error = StorageError>>;
    fn index(entry: LogEntry) -> Box<dyn Future<Item = (), Error = StorageError>>;
}