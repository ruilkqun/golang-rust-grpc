use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("proto");
}

use hello::hello_service_server::{HelloService,HelloServiceServer};
use hello::{HelloRequest,HelloResponse};

#[derive(Debug,Default)]
pub struct MyHello {}

#[tonic::async_trait]
impl HelloService for MyHello {
    async fn hello_world(&self,request:Request<HelloRequest>) -> Result<Response<HelloResponse>, Status>{
        println!("获取到请求：{:?}",request);
        let reply = HelloResponse {
            response: format!("收到请求:{} 回复:Successful! This is grpc server!",request.into_inner().request).into()
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8888".parse()?;
    let h = MyHello::default();

    Server::builder().add_service(HelloServiceServer::new(h)).serve(addr).await?;

    Ok(())
}

