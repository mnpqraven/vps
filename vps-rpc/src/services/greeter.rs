use proto_types::{greeter_server::Greeter, HelloReply, HelloRequest};
use tonic::{Request, Response, Status};
use tracing::Level;

#[derive(Debug, Default)]
pub struct GreeterRpc {}

#[tonic::async_trait]
impl Greeter for GreeterRpc {
    #[tracing::instrument(level=Level::DEBUG, ret(level=Level::INFO))]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let req = request.into_inner();
        tracing::info!("{:?}", req);

        let reply = HelloReply {
            message: format!("Hello {}!", req.name),
        };

        Ok(Response::new(reply))
    }
}
