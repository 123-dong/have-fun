mod db;
mod grpc;
mod handler;
mod rest;
mod server;

pub mod health {
    tonic::include_proto!("health");
}
