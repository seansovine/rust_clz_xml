fn main() {
  println!("Build script executing...");
  tonic_build::compile_protos("proto/clz_xml.proto")
    .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
