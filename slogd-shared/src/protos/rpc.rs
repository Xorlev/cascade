// This file is generated by rust-protobuf 2.0.6. Do not edit
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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\trpc.proto\x12\x05slogd\x1a\x1cgoogle/api/annotations.proto\x1a\ndata\
    .proto2\xf6\x02\n\rStructuredLog\x12Y\n\nAppendLogs\x12\x14.slogd.Append\
    Request\x1a\x15.slogd.AppendResponse\"\x1e\x82\xd3\xe4\x93\x02\x18\"\x13\
    /topic/{topic}/logs:\x01*\x12U\n\x07GetLogs\x12\x15.slogd.GetLogsRequest\
    \x1a\x16.slogd.GetLogsResponse\"\x1b\x82\xd3\xe4\x93\x02\x15\x12\x13/top\
    ic/{topic}/logs\x12_\n\nStreamLogs\x12\x15.slogd.GetLogsRequest\x1a\x16.\
    slogd.GetLogsResponse\"\x20\x82\xd3\xe4\x93\x02\x1a\x12\x18/topic/{topic\
    }/logstream0\x01\x12R\n\nListTopics\x12\x18.slogd.ListTopicsRequest\x1a\
    \x19.slogd.ListTopicsResponse\"\x0f\x82\xd3\xe4\x93\x02\t\x12\x07/topics\
    B\x18\n\nslog.protoP\x01\xa0\x01\x01Z\x05protob\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
