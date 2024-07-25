use core::fmt;

use protos::helloworld::v1::{
    greeter_service_client::GreeterServiceClient, SayHelloRequest, SayHelloRequestWrapper,
    SayHelloResponse,
};
use serde_wasm_bindgen::{from_value, to_value};
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Object;

#[wasm_bindgen]
pub struct GreeterClient {
    client: GreeterServiceClient<Client>,
}

use struct_accessor_macro::Accessors;

#[wasm_bindgen]
pub struct Toto {
    bar: String,
}

#[wasm_bindgen]
impl GreeterClient {
    pub fn new(url: String) -> GreeterClient {
        GreeterClient {
            client: GreeterServiceClient::new(Client::new(url)),
        }
    }

    pub async fn say_hello(
        &mut self,
        request: SayHelloRequestWrapper,
    ) -> Result<SayHelloResponse, JsError> {
        let test = Toto {
            bar: "test".to_string(),
        };

        // Call the gRPC method
        let response = self
            .client
            .say_hello(request.into_inner())
            .await
            .map_err(|err| JsError::from(err))?;

        // Extract the inner response from tonic::Response
        let inner_response = response.into_inner();
        println!("Got response message: {:?}", test.bar);
        // Serialize the response back into a JsValue
        //let response_js_value = to_value(&inner_response).map_err(|err| JsError::from(err))?;

        Ok(inner_response)
    }
}

#[wasm_bindgen]
pub async fn test_message() -> Result<String, JsError> {
    let url = "http://[::1]:50051";
    let wasm_client = Client::new(url.to_string());

    let mut h_client = GreeterServiceClient::new(wasm_client);

    let request = tonic::Request::new(SayHelloRequest {
        name: "Tonic".into(),
    });

    let response = h_client.say_hello(request).await;

    if let Ok(response) = response {
        let response = response.into_inner().message;
        println!("Got response message: {:?}", response);
        Ok(response)
    } else {
        println!("Error: {:?}", response);
        Err(JsError::new("Error"))
    }
}
//use serde::{Deserialize, Serialize};
//#[derive(Deserialize, Serialize)]
//pub struct SayHelloRequestBis {
//    pub name: ::prost::alloc::string::String,
//}

//impl wasm_bindgen::describe::WasmDescribe for SayHelloRequestBis {
//    fn describe() {
//        Object::describe();
//    }
//}

//impl wasm_bindgen::convert::FromWasmAbi for SayHelloRequestBis {
//    type Abi = <JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;

//    unsafe fn from_abi(js: Self::Abi) -> Self {
//        let js_value = <JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(js);
//        from_value(js_value).unwrap()
//    }
//}

//impl wasm_bindgen::convert::IntoWasmAbi for SayHelloRequestBis {
//    type Abi = <JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;

//    fn into_abi(self) -> Self::Abi {
//        let js_value = to_value(&self).unwrap();
//        <JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(js_value)
//    }
//}

#[derive(Debug)]
pub struct JsErrorWrapper {
    inner: tonic::Status,
}

impl fmt::Display for JsErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl std::error::Error for JsErrorWrapper {}

impl From<tonic::Status> for JsErrorWrapper {
    fn from(status: tonic::Status) -> Self {
        JsErrorWrapper { inner: status }
    }
}
