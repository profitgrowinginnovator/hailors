use autocxx_build::Builder;
use std::env;
//use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=OUT_DIR");
    println!("cargo:rerun-if-changed=src/device_api_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/device_api_wrapper.hpp");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let hailo_include_path = env::var("HAILORT_INCLUDE_PATH")
        .unwrap_or_else(|_| "/usr/include/hailo".to_string());


    //let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());  // Ensure output path for autocxx
    let bindings = Builder::new("src/lib.rs", &[&hailo_include_path, "src/"])
        .extra_clang_args(&[
            "-I/usr/include",
            "-I/usr/include/hailo",
            "-std=c++17",
            "-fno-rtti",
            "-fno-exceptions",
        ])
        .build()
        .expect("Failed to generate bindings");

    // Link with the hailort C++ library
    println!("cargo:rerun-if-changed=src/custom_wrapper.hpp");

    println!("cargo:rustc-link-search=native=/usr/lib");  // Path to the library
    println!("cargo:rustc-link-lib=hailort");

    // Compile C++ implementation
    cc::Build::new()
        .cpp(true)
        .file("src/device_api_wrapper.cpp")
        .include("src/") // For local includes
        .include(hailo_include_path) // For Hailo includes
        .flag("-std=c++17")
        .flag("-fno-rtti")
        .flag("-fno-exceptions")
        .warnings(true)
        .compile("hailors_cpp");

    println!("Bindings generated at {:?}", bindings);
}
