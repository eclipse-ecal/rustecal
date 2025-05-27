use std::{env, path::PathBuf};

fn main() {
    if std::env::var("DOCS_RS").is_ok() || std::env::var("CARGO_DOC").is_ok() {
        println!("cargo:warning=Skipping bindgen during documentation");
        return;
    }
    // Prepare bindgen builder, force all enums to 32-bit signed (MSVC ABI)
+    let mut builder = bindgen::Builder::default()
+       .clang_arg("-fms-compatibility")
        .header("wrapper.h")
        .allowlist_function("eCAL_.*")
        .allowlist_type("eCAL_.*")
        .allowlist_var("eCAL_.*")
        .layout_tests(false)
        .generate_comments(true)
        .derive_default(true)
        .wrap_unsafe_ops(true);

    if cfg!(target_os = "windows") {
        // --- Windows: Use ECAL_HOME ---
        let ecal_home = env::var("ECAL_HOME")
            .expect("ECAL_HOME environment variable must be set on Windows");
        let include_path = format!("{}/include", ecal_home);
        let lib_path = format!("{}/lib", ecal_home);

        println!("cargo:rustc-link-search=native={}", lib_path);
        println!("cargo:rustc-link-lib=static=ecal_core_c");

        builder = builder.clang_arg(format!("-I{}", include_path));

        // Debug info
        println!("cargo:warning=Building on Windows");
        println!("cargo:warning=Using ECAL_HOME = {}", ecal_home);
    } else if cfg!(target_os = "linux") {
        // --- Linux: Assume system-wide install ---
        println!("cargo:rustc-link-lib=dylib=ecal_core_c");
        println!("cargo:rustc-link-search=native=/usr/local/lib"); // Or /usr/lib if needed

        builder = builder
            .clang_arg("-I/usr/include")
            .clang_arg("-I/usr/local/include");

        // Debug info
        println!("cargo:warning=Building on Linux");
    } else {
        panic!("Unsupported platform for rustecal-sys build");
    }

    // Final bindgen output
    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
