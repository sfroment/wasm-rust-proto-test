import * as grpcWeb from 'grpc-web';

import * as helloworld_v1_helloworld_pb from '../../helloworld/v1/helloworld_pb'; // proto import: "helloworld/v1/helloworld.proto"


export class GreeterServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  sayHello(
    request: helloworld_v1_helloworld_pb.SayHelloRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: helloworld_v1_helloworld_pb.SayHelloResponse) => void
  ): grpcWeb.ClientReadableStream<helloworld_v1_helloworld_pb.SayHelloResponse>;

}

export class GreeterServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  sayHello(
    request: helloworld_v1_helloworld_pb.SayHelloRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<helloworld_v1_helloworld_pb.SayHelloResponse>;

}

