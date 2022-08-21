#![allow(clippy::all)]

tonic::include_proto!("waves");

pub mod events {
    tonic::include_proto!("waves.events");

    pub mod grpc {
        tonic::include_proto!("waves.events.grpc");
    }
}

pub mod node {
    pub mod grpc {
        tonic::include_proto!("waves.node.grpc");
    }
}
