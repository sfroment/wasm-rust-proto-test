//use tonic::transport::Server;

//include!("../gen/mod.rs");

//#[derive(Default)]
//pub struct MyGreeter {}

//#[tonic::async_trait]
//impl helloworld::v1::greeter_service_server::GreeterService for MyGreeter {
//    async fn say_hello(
//        &self,
//        request: tonic::Request<helloworld::v1::SayHelloRequest>,
//    ) -> Result<tonic::Response<helloworld::v1::SayHelloResponse>, tonic::Status> {
//        let request = request.into_inner();
//        println!("Got a request with name: {:?}", request.name);
//        let reply = helloworld::v1::SayHelloResponse {
//            message: format!("Hello {}!", request.name).into(),
//        };

//        Ok(tonic::Response::new(reply))
//    }
//}

//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let addr = "[::1]:9999".parse().unwrap();
//    let greeter = MyGreeter::default();

//    Server::builder()
//        .add_service(helloworld::v1::greeter_service_server::GreeterServiceServer::new(greeter))
//        .serve(addr)
//        .await?;

//    Ok(())
//}
