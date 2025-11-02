fn main() {
    println!("cargo:rerun-if-changed=src/start.S");
    cc::Build::new().file("src/start.S").compile("start");
}
