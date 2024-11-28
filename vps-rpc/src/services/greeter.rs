use greeter_server::Greeter;
use tonic::{Request, Response, Status};

tonic::include_proto!("helloworld");
pub const HELLO_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("helloworld_descriptor");

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
