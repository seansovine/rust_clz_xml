pub mod parser_thread;

pub mod grpc;

/// TODO: Spawn gRPC server and implement service to run
/// parser and stream results to client.
fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Hello, world from the gRPC server program!\n");

  return Ok(());
}
