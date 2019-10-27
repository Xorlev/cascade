fn main() {
    tonic_build::compile_protos("../proto/config.proto").unwrap();
    tonic_build::compile_protos("../proto/data.proto").unwrap();
    tonic_build::compile_protos("../proto/rpc.proto").unwrap();
}
