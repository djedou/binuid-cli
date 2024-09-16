use crate::binuid_grpc::{
    binuid_grpc_server::BinuidGrpc, 
    HelloRequest, HelloReply, RouteRequest, RouteReply
};
use tonic::{
    Request, Response, Status
};
use std::{
    env,
    fs::File,
    io::Read
};

type ServerResult<T> = Result<Response<T>, Status>;


pub struct GrpcService;

impl GrpcService {
    pub async fn new() -> Self {
        GrpcService
    }

    pub fn read_file(&self, route: &str) -> Result<Vec<u8>, String> {
        let path = match route.ends_with("/") {
            true => format!("{}index.bin", route),
            false => format!("{}.bin", route)
        };

        match env::current_dir() {
            Ok(dir) => {
                let path = format!("{}/pkg/app{}", dir.display(), path);
                let formated_path = path.replace("\\", "/");

                match File::open(formated_path) {
                    Ok(mut file) => {
                        let mut data = vec![];
                        match file.read_to_end(&mut data) {
                            Ok(_) => Ok(data),
                            Err(_) => Err(format!("No module for route {}", route))
                        }
                    },
                    Err(_) => Err(format!("No module for route {}", route))
                }
            },
            Err(_) => {
                Err("Unkonwed directory!".to_owned())
            }
        }
    }
}

#[tonic::async_trait]
impl BinuidGrpc for GrpcService {

    async fn say_hello(&self, count: Request<HelloRequest>) -> ServerResult<HelloReply> {
        println!("count: {count:?}");
        Ok(Response::new(HelloReply {
            message: "Bravo djedou Arnaud".to_owned()
        }))
    }

    async fn route(&self, req: Request<RouteRequest>) -> ServerResult<RouteReply> {
        match self.read_file(&req.into_inner().route) {
            Ok(data) => {
                Ok(Response::new(RouteReply {data}))
            },
            Err(e) => {
                Err(Status::unknown(e))
            }
        }
    }
}