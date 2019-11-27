use crate::slogd_proto::LogEntry;
use crate::storage::segment::Segment;
use crate::storage::{Log, LogQuery, Offset, StorageResult, TopicName};
use async_std::fs::{DirEntry, ReadDir};
use async_std::path::PathBuf;
use async_std::sync::{Arc, RwLock};
use async_trait::async_trait;
use futures::StreamExt;
use std::collections::BTreeMap;

/// Topic is the underlying structure which manages multiple LogSegments. As each segment
/// becomes too large or has been open for too long, the Topic closes that segment and
/// opens a new segment. Topic implements the Log methods as it must decide which segment(s)
/// are necessary to query to answer a given request.
pub struct Topic {
    // TODO, make this many segments
    segments: Vec<Segment>,
}

impl Topic {
    pub async fn create(path: &PathBuf) -> StorageResult<Topic> {
        // Read all segments
        let mut dir: ReadDir = async_std::fs::read_dir(path).await?;
        let mut segments = BTreeMap::new();
        while let Some(value) = dir.next().await {
            let dir_entry: DirEntry = value?;
            let filename: String = dir_entry.file_name().to_string_lossy().to_string();

            if filename.ends_with(".log") {
                let start_offset = filename
                    .split(".")
                    .next()
                    .unwrap()
                    .parse::<Offset>()
                    .unwrap();

                let segment = Segment::open(dir_entry.path(), start_offset).await?;
                segments.insert(start_offset, segment);
            }
        }

        let segments = segments.into_iter().map(|(_, segment)| segment).collect();

        Ok(Topic { segments })
    }
}

#[async_trait]
impl Log for Topic {
    async fn append(&mut self, entries: Vec<LogEntry>) -> StorageResult<Offset> {
        unimplemented!()
    }

    async fn read(&self, query: LogQuery) -> StorageResult<Vec<LogEntry>> {
        self.segments[0].read(query).await
    }

    async fn flush(&mut self) {
        unimplemented!()
    }

    fn close(&mut self) {
        unimplemented!()
    }
}
