// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_STRUCTURED_LOG_APPEND_LOGS: ::grpcio::Method<super::data::AppendRequest, super::data::AppendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/slogd.StructuredLog/AppendLogs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STRUCTURED_LOG_GET_LOGS: ::grpcio::Method<super::data::GetLogsRequest, super::data::GetLogsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/slogd.StructuredLog/GetLogs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STRUCTURED_LOG_STREAM_LOGS: ::grpcio::Method<super::data::GetLogsRequest, super::data::GetLogsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/slogd.StructuredLog/StreamLogs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STRUCTURED_LOG_LIST_TOPICS: ::grpcio::Method<super::data::ListTopicsRequest, super::data::ListTopicsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/slogd.StructuredLog/ListTopics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct StructuredLogClient {
    client: ::grpcio::Client,
}

impl StructuredLogClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        StructuredLogClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn append_logs_opt(&self, req: &super::data::AppendRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::data::AppendResponse> {
        self.client.unary_call(&METHOD_STRUCTURED_LOG_APPEND_LOGS, req, opt)
    }

    pub fn append_logs(&self, req: &super::data::AppendRequest) -> ::grpcio::Result<super::data::AppendResponse> {
        self.append_logs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn append_logs_async_opt(&self, req: &super::data::AppendRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::data::AppendResponse>> {
        self.client.unary_call_async(&METHOD_STRUCTURED_LOG_APPEND_LOGS, req, opt)
    }

    pub fn append_logs_async(&self, req: &super::data::AppendRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::data::AppendResponse>> {
        self.append_logs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_logs_opt(&self, req: &super::data::GetLogsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::data::GetLogsResponse> {
        self.client.unary_call(&METHOD_STRUCTURED_LOG_GET_LOGS, req, opt)
    }

    pub fn get_logs(&self, req: &super::data::GetLogsRequest) -> ::grpcio::Result<super::data::GetLogsResponse> {
        self.get_logs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_logs_async_opt(&self, req: &super::data::GetLogsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::data::GetLogsResponse>> {
        self.client.unary_call_async(&METHOD_STRUCTURED_LOG_GET_LOGS, req, opt)
    }

    pub fn get_logs_async(&self, req: &super::data::GetLogsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::data::GetLogsResponse>> {
        self.get_logs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stream_logs_opt(&self, req: &super::data::GetLogsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::data::GetLogsResponse>> {
        self.client.server_streaming(&METHOD_STRUCTURED_LOG_STREAM_LOGS, req, opt)
    }

    pub fn stream_logs(&self, req: &super::data::GetLogsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::data::GetLogsResponse>> {
        self.stream_logs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_topics_opt(&self, req: &super::data::ListTopicsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::data::ListTopicsResponse> {
        self.client.unary_call(&METHOD_STRUCTURED_LOG_LIST_TOPICS, req, opt)
    }

    pub fn list_topics(&self, req: &super::data::ListTopicsRequest) -> ::grpcio::Result<super::data::ListTopicsResponse> {
        self.list_topics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_topics_async_opt(&self, req: &super::data::ListTopicsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::data::ListTopicsResponse>> {
        self.client.unary_call_async(&METHOD_STRUCTURED_LOG_LIST_TOPICS, req, opt)
    }

    pub fn list_topics_async(&self, req: &super::data::ListTopicsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::data::ListTopicsResponse>> {
        self.list_topics_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait StructuredLog {
    fn append_logs(&self, ctx: ::grpcio::RpcContext, req: super::data::AppendRequest, sink: ::grpcio::UnarySink<super::data::AppendResponse>);
    fn get_logs(&self, ctx: ::grpcio::RpcContext, req: super::data::GetLogsRequest, sink: ::grpcio::UnarySink<super::data::GetLogsResponse>);
    fn stream_logs(&self, ctx: ::grpcio::RpcContext, req: super::data::GetLogsRequest, sink: ::grpcio::ServerStreamingSink<super::data::GetLogsResponse>);
    fn list_topics(&self, ctx: ::grpcio::RpcContext, req: super::data::ListTopicsRequest, sink: ::grpcio::UnarySink<super::data::ListTopicsResponse>);
}

pub fn create_structured_log<S: StructuredLog + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STRUCTURED_LOG_APPEND_LOGS, move |ctx, req, resp| {
        instance.append_logs(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STRUCTURED_LOG_GET_LOGS, move |ctx, req, resp| {
        instance.get_logs(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_STRUCTURED_LOG_STREAM_LOGS, move |ctx, req, resp| {
        instance.stream_logs(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STRUCTURED_LOG_LIST_TOPICS, move |ctx, req, resp| {
        instance.list_topics(ctx, req, resp)
    });
    builder.build()
}
