
use std::env;


use autocxx_build::Builder;

fn main() {
    println!("cargo:rerun-if-env-changed=OUT_DIR");

    println!("cargo:rustc-link-lib=hailort"); 
    println!("cargo:rustc-link-search=native=/usr/lib"); 
    println!("cargo:rerun-if-changed=src/device_api_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/device_api_wrapper.hpp");

    
    let hailo_include_path = env::var("HAILORT_INCLUDE_PATH")
    .unwrap_or_else(|_| "/usr/include/hailo".to_string());


    let mut build = Builder::new("src/lib.rs", &[&hailo_include_path, "src/"])
        .extra_clang_args(&["-I/usr/include", "-I/usr/include/hailo", "-std=c++17", "-fno-rtti", "-fno-exceptions"])
        .build()
        .unwrap();

        println!("cargo:rustc-link-search=native=src/lib.rs");
        println!("cargo:rustc-link-lib=hailort");
        build
        .flag_if_supported("-std=c++17")
        .file("src/device_api_wrapper.cpp")
        .compile("hailors");

        println!("cargo:rerun-if-changed=src/lib.rs");

}
