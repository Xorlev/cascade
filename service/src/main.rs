use failure::Fail;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status, Streaming};
use tower_service::Service;

use slogd_proto::{
    server, AppendRequest, AppendResponse, GetLogsRequest, GetLogsResponse, ListTopicsRequest,
    ListTopicsResponse, LogEntry,
};

use crate::storage::{Log, LogQuery, StorageError, Topic, TopicManager, TopicName};
use async_std::sync::{Mutex, RwLock};

mod storage;

pub mod slogd_proto {
    include!(concat!(env!("OUT_DIR"), concat!("/slogd.rs")));
}

struct StructuredLogService {
    topic_manager: Arc<RwLock<TopicManager>>,
}

#[tonic::async_trait]
impl server::StructuredLog for StructuredLogService {
    async fn append_logs(
        &self,
        request: Request<AppendRequest>,
    ) -> Result<Response<AppendResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
    type AppendLogsStreamStream = mpsc::Receiver<Result<AppendResponse, Status>>;

    async fn append_logs_stream(
        &self,
        request: Request<Streaming<AppendRequest>>,
    ) -> Result<Response<Self::AppendLogsStreamStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_logs(
        &self,
        request: Request<GetLogsRequest>,
    ) -> Result<Response<GetLogsResponse>, Status> {
        let request = request.into_inner();
        let topic = self.get_topic(&request.topic).await?;

        let logs = {
            let read_locked_topic = topic.read().await;
            read_locked_topic.read(LogQuery::builder().build()).await?
        };

        Ok(Response::new(GetLogsResponse { logs }))
    }
    type GetLogsStreamStream = mpsc::Receiver<Result<LogEntry, Status>>;
    async fn get_logs_stream(
        &self,
        request: Request<GetLogsRequest>,
    ) -> Result<Response<Self::GetLogsStreamStream>, Status> {
        let response = self.get_logs(request).await?;

        // Until we have a cursor implementation, this just delegates to get_logs and ends the
        // stream.
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for log in response.into_inner().logs {
                tx.send(Ok(log)).await.unwrap();
            }
        });

        Ok(Response::new(rx))
    }
    async fn list_topics(
        &self,
        request: Request<ListTopicsRequest>,
    ) -> Result<Response<ListTopicsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr = "127.0.0.1:10000".parse().unwrap();

    println!("Listening on: {}", addr);
    let tm = TopicManager::new().await.map_err(|e| e.compat())?;

    let structured_log_service = StructuredLogService {
        topic_manager: Arc::new(RwLock::new(tm)),
    };
    let svc = server::StructuredLogServer::new(structured_log_service);

    let mut builder = Server::builder();
    builder.interceptor_fn(|svc, req| {
        println!("request={:?}", req);
        svc.call(req)
    });
    builder.add_service(svc).serve(addr).await?;

    Ok(())
}

impl From<StorageError> for Status {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::IoError(e) => Status::new(Code::Internal, e.to_string()),
            _ => Status::new(Code::Unknown, "Status mapping unimplemented."),
        }
    }
}

impl StructuredLogService {
    async fn get_topic(&self, topic: &TopicName) -> Result<Arc<RwLock<Topic>>, Status> {
        self.topic_manager
            .read()
            .await
            .topic(topic)
            .ok_or_else(|| Status::new(Code::NotFound, format!("Topic '{}' not found.", topic)))
    }
}
