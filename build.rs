fn main() {
    prost_build::compile_protos(&["pipeline.proto"], &["./"]).unwrap();
}
