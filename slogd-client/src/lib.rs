extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate slogd_shared;

use std::sync::Arc;

use grpcio::*;

use slogd_shared::protos::rpc_grpc::StructuredLogClient;
use std::time::Duration;

pub fn new(host: &str) -> StructuredLogClient {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env)
        .initial_reconnect_backoff(Duration::from_millis(250))
        .max_reconnect_backoff(Duration::from_millis(1000))
        .keepalive_timeout(Duration::from_millis(1000))
        .connect(host);

    StructuredLogClient::new(channel)
}