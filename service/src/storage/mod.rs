use std::collections::HashMap;
use std::sync::Arc;

use async_std::sync::RwLock;
use failure::Fail;

use async_trait::async_trait;
pub use query::LogQuery;
pub use topic::Topic;

use crate::slogd_proto::LogEntry;
use async_std::fs::{DirEntry, ReadDir};
use async_std::path::PathBuf;
use futures::StreamExt;
use std::collections::hash_map::Entry;

mod file;
mod index;
mod query;
mod segment;
mod topic;

#[derive(Debug, Fail)]
pub enum StorageError {
    #[fail(display = "an unknown error occurred.")]
    UnknownError,
    #[fail(display = "io error: {}", _0)]
    IoError(std::io::Error),
}

pub type Offset = u64;
pub type StorageResult<T> = Result<T, StorageError>;
pub type TopicName = String;

#[async_trait]
pub trait Log {
    async fn append(&mut self, entries: Vec<LogEntry>) -> StorageResult<Offset>;
    async fn read(&self, query: LogQuery) -> StorageResult<Vec<LogEntry>>;
    async fn flush(&mut self);
    fn close(&mut self);
}

/// LogManager maintains topics, including running periodic maintenance tasks.
pub struct TopicManager {
    base_path: PathBuf,
    topics: HashMap<TopicName, Arc<RwLock<Topic>>>,
}

impl TopicManager {
    pub async fn new() -> StorageResult<TopicManager> {
        let mut topics = HashMap::new();

        // Load topics from disk.
        let mut dir: ReadDir = async_std::fs::read_dir("data").await?;
        while let Some(value) = dir.next().await {
            let dir_entry: DirEntry = value?;
            let dir_name: String = dir_entry.file_name().to_string_lossy().to_string();
            let topic = Topic::create(&dir_entry.path()).await?;
            topics.insert(dir_name, Arc::new(RwLock::new(topic)));
        }

        Ok(TopicManager {
            base_path: PathBuf::from("data"),
            topics,
        })
    }

    pub fn topic(&self, topic_name: &TopicName) -> Option<Arc<RwLock<Topic>>> {
        self.topics.get(topic_name).cloned()
    }

    pub async fn topic_or_create(
        &mut self,
        topic_name: TopicName,
    ) -> StorageResult<Arc<RwLock<Topic>>> {
        match self.topics.entry(topic_name.clone()) {
            Entry::Occupied(v) => Ok(v.get().clone()),
            Entry::Vacant(_) => {
                let mut topic_path = self.base_path.to_path_buf();
                topic_path.push(topic_name);
                let topic = Topic::create(&topic_path).await?;
                Ok(Arc::new(RwLock::new(topic)))
            }
        }
    }
}
