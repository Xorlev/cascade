/// LogQuery is a Rust representation of a log query. A log query has three primary parts:
///
/// 1) A starting offset. This can be either a special offset (earliest, latest), an actual u64
///    offset, or some kind of index query (such as by timestamp).
///
/// 2) A filter. Currently this is just an annotation post-filter.
///
/// 3) Options. This includes behavior such as a maximum number of messages to return.
pub struct LogQuery {
    start_at: StartAt,
}

impl LogQuery {
    pub fn builder() -> LogQueryBuilder {
        LogQueryBuilder::new()
    }

    fn from_builder(mut builder: LogQueryBuilder) -> LogQuery {
        LogQuery {
            start_at: builder.start_at,
        }
    }
}

pub struct LogQueryBuilder {
    start_at: StartAt,
}

impl LogQueryBuilder {
    fn new() -> LogQueryBuilder {
        LogQueryBuilder {
            start_at: StartAt::Latest,
        }
    }

    pub fn start_at(mut self, start_at: StartAt) -> LogQueryBuilder {
        self.start_at = start_at;
        self
    }

    pub fn build(mut self) -> LogQuery {
        LogQuery::from_builder(self)
    }
}

pub enum StartAt {
    Latest,
    Earliest,
}
