pub mod grpc;
pub mod parser_thread;

pub mod clz_xml {
  tonic::include_proto!("clz_xml");
}

use tonic::transport::Server;

use clz_xml::clz_xml_server;

/// TODO: Spawn gRPC server and implement service to run
/// parser and stream results to client.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Hello, world from the gRPC server program!\n");

  let addr = "[::1]:10000".parse().unwrap();

  let clz_xml_svc = grpc::ClzXmlService;

  let svc = clz_xml_server::ClzXmlServer::new(clz_xml_svc);

  Server::builder().add_service(svc).serve(addr).await?;

  return Ok(());
}
