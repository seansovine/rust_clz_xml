pub mod clz_xml {
  tonic::include_proto!("clz_xml");
}

use tonic::Request;

use clz_xml::{clz_xml_client::ClzXmlClient, File};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Hello, world from the gRPC client!");

  let mut client = ClzXmlClient::connect("http://[::1]:10000").await?;

  let response = client
    .parse(Request::new(File {
      path: "".to_string(),
    }))
    .await?;

  println!("RESPONSE = {:?}", response);

  Ok(())
}
