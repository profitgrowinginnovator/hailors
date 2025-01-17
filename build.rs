use cc::Build;
fn main() {
    println!("cargo:rerun-if-changed=src/device_api_wrapper.cpp");
    println!("cargo:rerun-if-changed=src/device_api_wrapper.hpp");
    Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("src/device_api_wrapper.cpp")
        .include("/usr/include/hailo")
        .compile("hailors");
    println!("cargo:rustc-link-lib=hailort");
}
