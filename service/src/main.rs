extern crate env_logger;
extern crate futures;
extern crate grpcio;
extern crate shared;
extern crate protobuf;
#[macro_use] extern crate log;

use std::sync::Arc;
use std::io::Read;
use std::time::Instant;
use std::{io, thread};

use futures::*;
use futures::sync::oneshot;
use grpcio::*;
use shared::protos::data::*;
use shared::protos::rpc_grpc::*;
use shared::protos::rpc_grpc::{self, StructuredLogService};


#[derive(Clone)]
struct StructuredLog {}

impl StructuredLogService for StructuredLog {
    fn append_logs(&self, ctx: RpcContext, req: AppendRequest, sink: UnarySink<AppendResponse>) {
        unimplemented!()
    }

    fn stream_logs(&self, ctx: RpcContext, req: StreamLogsRequest, sink: ServerStreamingSink<StreamLogsResponse>) {
        println!("{:?}", req);

        let logs = vec![
            (StreamLogsResponse::new(), WriteFlags::default())
        ];

        sink.send_all(stream::iter_ok::<_, Error>(logs));
    }
}

fn main() {
    env_logger::init();

    let instance = StructuredLog{};
    let env = Arc::new(Environment::new(2));
    let service = rpc_grpc::create_structured_log_service(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
