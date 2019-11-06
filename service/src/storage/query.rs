/// LogQuery is a Rust representation of a log query. A log query has three primary parts:
///
/// 1) A starting offset. This can be either a special offset (earliest, latest), an actual u64
///    offset, or some kind of index query (such as by timestamp).
///
/// 2) A filter. Currently this is just an annotation post-filter.
///
/// 3) Options. This includes behavior such as a maximum number of messages to return.
pub struct LogQuery;
