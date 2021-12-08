
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/hello.c");
    // std::env::set_var("CC", "gcc");
    // std::env::set_var("AR", "ar");

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/libtelnet.c")
        .compile("libtelnet");
}