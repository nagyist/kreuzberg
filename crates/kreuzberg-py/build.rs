use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    // Try to locate kreuzberg-ffi library built alongside this crate
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = PathBuf::from(&cargo_manifest_dir);

    // Prefer host target layout, but include target-triple layout for cross builds.
    if let Some(workspace_root) = manifest_path.parent().and_then(|p| p.parent()) {
        let host_deps_dir = workspace_root.join("target").join(&profile).join("deps");
        let host_lib_dir = workspace_root.join("target").join(&profile);
        let target_deps_dir = workspace_root.join("target").join(&target).join(&profile).join("deps");
        let target_lib_dir = workspace_root.join("target").join(&target).join(&profile);

        for dir in [host_deps_dir, host_lib_dir, target_deps_dir, target_lib_dir] {
            if dir.exists() {
                println!("cargo:rustc-link-search=native={}", dir.display());
            }
        }
    }

    // Link the kreuzberg-ffi library
    // When kreuzberg-ffi is built, its symbols become available for linking
    println!("cargo:rustc-link-lib=static=kreuzberg_ffi");

    if target.contains("darwin") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
