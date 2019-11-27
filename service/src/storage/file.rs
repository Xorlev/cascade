use crate::storage::StorageResult;
use async_std::path::{Path, PathBuf};
use async_std::task::spawn_blocking;
use buffered_offset_reader::{BufOffsetReader, OffsetRead, OffsetReadMut};
use std::fs::File;
use std::io::BufRead;
use std::sync::{Arc, RwLock};
use tokio::io::Error;

pub struct LogFile {
    inner: Arc<RwLock<File>>,
}

impl LogFile {
    pub async fn open<P: AsRef<Path>>(path: P) -> StorageResult<LogFile> {
        let path = path.as_ref().to_owned().into_os_string();

        let f = spawn_blocking(move || File::open(path)).await?;

        Ok(LogFile {
            inner: Arc::new(RwLock::new(f)),
        })
    }

    pub async fn read_at(&self, buf: &mut [u8], offset: u64) -> StorageResult<usize> {
        let file = self.inner.clone();
        let read_bytes: usize = spawn_blocking(move || {
            let f = file.read().unwrap();
            let read_bytes: usize = f.read_at(&mut buf, offset).unwrap();
            read_bytes
        })
        .await;

        Ok(read_bytes)
    }
}
