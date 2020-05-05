extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/proto";
    protoc_grpcio::compile_grpc_protos(&["stock.proto"], &[proto_root], &proto_root, None).unwrap();
}
