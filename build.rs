fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    let dbc_path = "dbc/e46.dbc";
    let dbc_file = std::fs::read(dbc_path).unwrap();
    println!("cargo:rerun-if-changed={}", dbc_path);

    let mut out_file = std::io::BufWriter::new(std::fs::File::create("src/messages.rs").unwrap());
    dbc_codegen::codegen("example.dbc", &dbc_file, &mut out_file, true)
        .expect("Generating CAN messages failed");
}
