use proto::v1::DESCRIPTOR_SET;
use proto::v1::chat::chat_service_server::{ChatService, ChatServiceServer};
use proto::v1::chat::{ChatRequest, ChatResponse};
use shared::utils;
use std::pin::Pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status, transport::Server};
use uuid::Uuid;

#[derive(Debug, Default)]
struct MyChat;

type StreamResponse = Pin<Box<dyn Stream<Item = Result<ChatResponse, Status>> + Send>>;
type StreamResult<T> = Result<tonic::Response<T>, tonic::Status>;

#[tonic::async_trait]
impl ChatService for MyChat {
    type ChatStream = StreamResponse;

    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatRequest>>,
    ) -> StreamResult<Self::ChatStream> {
        println!("New client connected from {:?}", request.remote_addr());

        let mut inbound = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(req) = inbound.next().await {
                let msg = req?;
                println!(
                    "[{}] {}: {}",
                    msg.room_id, msg.sender_id, msg.message_text
                );

                // echo back with a generated message_id
                yield ChatResponse {
                    room_id: msg.room_id.clone(),
                    sender_id: msg.sender_id.clone(),
                    message_text: format!("echo: {}", msg.message_text),
                    message_id: Uuid::new_v4().to_string(),
                };
            }
        };
        Ok(Response::new(Box::pin(output) as Self::ChatStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50057".parse()?;
    let svc = MyChat::default();
    let reflection = utils::init_refl(DESCRIPTOR_SET)?;
    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(reflection)
        .add_service(ChatServiceServer::new(svc))
        .serve_with_shutdown(addr, utils::shutdown_signal())
        .await?;

    Ok(())
}
