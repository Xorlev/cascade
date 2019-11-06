use core::pin::Pin;
use std::collections::HashMap;
use std::sync::Arc;

use async_std::sync::RwLock;
use failure::Fail;

use async_trait::async_trait;
pub use query::LogQuery;
pub use topic::Topic;

use crate::slogd_proto::LogEntry;

mod index;
mod query;
mod segment;
mod topic;

#[derive(Debug, Fail)]
pub enum StorageError {
    #[fail(display = "An unknown error occurred.")]
    UnknownError,
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
    topics: HashMap<TopicName, Arc<RwLock<Topic>>>,
}

impl TopicManager {
    pub fn new() -> TopicManager {
        let mut topics = HashMap::new();
        // Load topics from disk.

        TopicManager { topics }
    }

    pub fn topic(&mut self, topic_name: TopicName) -> Arc<RwLock<Topic>> {
        self.topics
            .entry(topic_name.clone())
            .or_insert_with(|| Arc::new(RwLock::new(Topic::create(topic_name))))
            .clone()
    }
}
