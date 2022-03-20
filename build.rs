use protobuf_codegen_pure::Codegen;

fn main() {
    Codegen::new()
    .input("src/protos/eventdata.proto")
    .include("src/protos")
    .out_dir("src/protos")
    .run()
    .expect("Codegen failed.");
}