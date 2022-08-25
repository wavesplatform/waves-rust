fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    tonic_build::configure().compile(
        &[
            "proto/waves/node/grpc/accounts_api.proto",
            "proto/waves/node/grpc/assets_api.proto",
            "proto/waves/node/grpc/blockchain_api.proto",
            "proto/waves/node/grpc/blocks_api.proto",
            "proto/waves/node/grpc/transactions_api.proto",
            "proto/waves/events/events.proto",
            "proto/waves/events/grpc/blockchain_updates.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
