fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = tonic_build::Config::new();
    config.protoc_executable(protobuf_src::protoc());
    tonic_build::configure().compile_protos_with_config(
        config,
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
