use libbpf_cargo::SkeletonBuilder;
use std::env;
use std::path::PathBuf;

const SRC: &str = "src/bpf/fentry.bpf.c";
const HDR: &str = "src/bpf/fentry.h";

fn main() {
    let mut out =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR must be set in build script"));
    out.push("fentry.skel.rs");
    SkeletonBuilder::new()
        .source(SRC)
        .clang_args("-Wno-compare-distinct-pointer-types")
        .build_and_generate(&out)
        .unwrap();
    
    out.pop();
    out.push("fentry.rs");
    let bindings = bindgen::Builder::default()
        .header(HDR)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate().unwrap();
    bindings.write_to_file(&out).unwrap();
    println!("cargo:rerun-if-changed={SRC}");
    println!("cargo:rerun-if-changed={HDR}");
}