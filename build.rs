use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    let neonucleus_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("neonucleus");

    let status = Command::new("zig")
        .arg("build")
        .arg("engine")
        .arg("-DnoEmu")
        .current_dir(&neonucleus_dir)
        .status()
        .expect("Failed to run zig build");

    assert!(status.success(), "Zig build failed");

    let out_dir = neonucleus_dir.join("zig-out/lib");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let nn_bindings = bindgen::builder()
        .header("neonucleus/src/neonucleus.h")
        .generate()
        .unwrap();

    nn_bindings.write_to_file(out.join("nn_bindings.rs")).unwrap();

    let lua_bindings = bindgen::builder()
        .header("neonucleus/foreign/lua54/lua.h")
        .generate()
        .unwrap();

    lua_bindings.write_to_file(out.join("lua_bindings.rs")).unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=neonucleus");
}