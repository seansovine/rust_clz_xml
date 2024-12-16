pub mod grpc;
pub mod parser_thread;

pub mod clz_xml {
  tonic::include_proto!("clz_xml");
}

use std::env;

use tonic::transport::Server;

use clz_xml::clz_xml_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Hello, world from the gRPC server program!\n");

  //
  let current_dir = env::current_dir().unwrap();
  let current_dir_str = current_dir.into_os_string().into_string().unwrap();
  println!("Current directory is: {current_dir_str}");
  //

  let clz_xml_svc = grpc::ClzXmlService;
  let svc = clz_xml_server::ClzXmlServer::new(clz_xml_svc);

  let addr = "0.0.0.0:10000".parse().unwrap();
  Server::builder().add_service(svc).serve(addr).await?;

  return Ok(());
}
