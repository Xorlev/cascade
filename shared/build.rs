extern crate protoc_rust;
extern crate protoc_grpcio;


fn main() {
    let proto_root = "../proto";

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &[
            "../proto/config.proto",
            "../proto/data.proto",
            "../proto/rpc.proto"
        ],
        includes: &[proto_root],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    }).expect("protoc");

    let proto_root = "../proto";

    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["../proto/rpc.proto"],
        &[proto_root],
        "src/protos"
    ).expect("Failed to compile gRPC definitions!");
}