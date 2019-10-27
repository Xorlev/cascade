use futures::{Stream, StreamExt};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;
use std::{io, thread};
use tokio::sync::mpsc;
use tonic::transport::Server;
use tower_service::Service;

use tonic::{Request, Response, Status};

pub mod slogd_proto {
    tonic::include_proto!("slogd");
}

use slogd_proto::{
    server, AppendRequest, AppendResponse, GetLogsRequest, GetLogsResponse, ListTopicsRequest,
    ListTopicsResponse,
};

use futures::*;
use tokio::prelude::*;

#[derive(Clone)]
struct StructuredLogService {}

#[tonic::async_trait]
impl server::StructuredLog for StructuredLogService {
    type StreamLogsStream = mpsc::Receiver<Result<GetLogsResponse, Status>>;

    async fn append_logs(
        &self,
        request: Request<AppendRequest>,
    ) -> Result<Response<AppendResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
    async fn get_logs(
        &self,
        request: Request<GetLogsRequest>,
    ) -> Result<Response<GetLogsResponse>, Status> {
        Ok(Response::new(GetLogsResponse {
            logs: vec![]
        }))
    }
    async fn stream_logs(
        &self,
        request: Request<GetLogsRequest>,
    ) -> Result<Response<Self::StreamLogsStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
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

    let structured_log_service = StructuredLogService {};
    let svc = server::StructuredLogServer::new(structured_log_service);

    let mut builder = Server::builder();
    builder.interceptor_fn(|svc, req| {
            println!("request={:?}", req);
            svc.call(req)
        });
    builder.serve(addr, svc).await?;

    Ok(())
}
