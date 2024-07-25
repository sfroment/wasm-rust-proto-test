include!("./gen/mod.rs");
use helloworld::v1::{greeter_service_client::GreeterServiceClient, SayHelloRequest};

use tonic_web_wasm_client::Client;
use crate::proto;

use proto::

pub async fn test_message() -> Result<String, String> {
    let url = "http://[::1]:9999";
    //
    let wasm_client = Client::new(url.to_string());

    let q_client = GreeterServiceClient::new(wasm_client);

    //let mut client = GreeterServiceClient::with_origin(svc, "http://[::1]:9999".try_into()?);

    //let request = tonic::Request::new(SayHelloRequest {
    //    name: "Tonic".into(),
    //});

    //let response = client.say_hello(request).await?;

    //println!("Got response message: {:?}", response.into_inner().message);

    Ok("toto".to_string())
}
