use std::{error::Error, time::Duration};

use http::header::HeaderName;
use protos::helloworld::v1::{
    greeter_service_server::GreeterService, greeter_service_server::GreeterServiceServer,
    SayHelloRequest, SayHelloResponse,
};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub struct GretterServer;

#[tonic::async_trait]
impl GreeterService for GretterServer {
    async fn say_hello(
        &self,
        request: Request<SayHelloRequest>,
    ) -> Result<Response<SayHelloResponse>, Status> {
        println!("echo");
        let request = request.into_inner();
        println!("Got a request with name: {:?}", request.name);
        let reply = SayHelloResponse {
            message: format!("Hello {}!", request.name).into(),
        };

        Ok(Response::new(reply))
    }
}

const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
    ["grpc-status", "grpc-message", "grpc-status-details-bin"];
const DEFAULT_ALLOW_HEADERS: [&str; 4] =
    ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let echo = GreeterServiceServer::new(GretterServer);

    println!("Echo server listening on: {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::mirror_request())
                .allow_credentials(true)
                .max_age(DEFAULT_MAX_AGE)
                .expose_headers(
                    DEFAULT_EXPOSED_HEADERS
                        .iter()
                        .cloned()
                        .map(HeaderName::from_static)
                        .collect::<Vec<HeaderName>>(),
                )
                .allow_headers(
                    DEFAULT_ALLOW_HEADERS
                        .iter()
                        .cloned()
                        .map(HeaderName::from_static)
                        .collect::<Vec<HeaderName>>(),
                ),
        )
        .layer(GrpcWebLayer::new())
        .add_service(echo)
        .serve(addr)
        .await?;

    Ok(())
}
