use autocxx_build::Builder;
use std::env;


fn main() {
    println!("Executing build.rs...");
    println!("cargo:rerun-if-changed=src/device_api_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/custom_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/device_api_wrapper.hpp");
    println!("cargo:rerun-if-changed=src/custom_wrapper.hpp");

    // Compile the C++ source files
    cc::Build::new()
        .cpp(true)
        .file("src/device_api_wrapper.cpp")
        .file("src/custom_wrapper.cpp")
        .flag("-std=c++17")
        .compile("device_wrappers");

    let hailo_include_path = env::var("HAILORT_INCLUDE_PATH")
        .unwrap_or_else(|_| "/usr/include/hailo".to_string());

    Builder::new("src/lib.rs", &["src/", &hailo_include_path])
        .extra_clang_args(&[
            "-std=c++17",
            "-fPIC",
            "-I/usr/include",
            "-I/usr/include/hailo",
        ])
        .build()
        .expect("Failed to generate bindings");

    println!("cargo:rustc-link-lib=hailort");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=tests/lib_test.rs");
}
