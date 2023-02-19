fn main() {
    println!("cargo:rerun-if-changed=src/capture.c");
    cc::Build::new().file("src/capture.c").compile("capture");
}
