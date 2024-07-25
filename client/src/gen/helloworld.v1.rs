// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SayHelloRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SayHelloResponse {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
include!("helloworld.v1.tonic.rs");
// @@protoc_insertion_point(module)