use std::env;
use std::process::Command;
use std::path::PathBuf;

fn opt_level_to_zig(opt_level: &str) -> &str {
    match opt_level {
        "0" => "Debug",
        "1" | "2" => "ReleaseSafe",
        "3" => "ReleaseFast",
        "s" | "z" => "ReleaseSmall",
        _ => "Debug", // fallback if unknown
    }
}

fn target_to_zig(target: &str) -> &str {
    match target {
        "x86_64-pc-windows-msvc" => "x86_64-windows-msvc",
        "x86_64-unknown-linux-gnu" => "x86_64-linux-gnu",
        "aarch64-apple-darwin" => "aarch64-macos",
        "wasm32-unknown-unknown" => "wasm32-freestanding",
        "wasm32-unknown-emscripten" => "wasm32-emscripten",
        // TODO: more mappings
        _ => target,
    }
}

fn main() {
    let neonucleus_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("neonucleus");

    let status = Command::new("zig")
        .arg("build")
        .arg("engine")
        .arg("-DnoEmu")
        .arg("-Dbaremetal")
        .arg(format!("-Dtarget={}", target_to_zig(&env::var("TARGET").unwrap())))
        .arg(format!("-Doptimize={}", opt_level_to_zig(&env::var("OPT_LEVEL").unwrap())))
        .current_dir(&neonucleus_dir)
        .status()
        .expect("Failed to run zig build");

    assert!(status.success(), "Zig build failed");

    let out_dir = neonucleus_dir.join("zig-out").join("lib");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let nn_bindings = bindgen::builder()
        .header("neonucleus/src/neonucleus.h")
        .clang_arg("-fvisibility=default")
        .blocklist_type("max_align_t")
        .generate()
        .unwrap();

    nn_bindings.write_to_file(out.join("nn_bindings.rs")).unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=neonucleus");
}