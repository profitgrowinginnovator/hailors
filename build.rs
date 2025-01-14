use cc::Build;
fn main() {
    Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("src/device_api_wrapper.cpp")
        .include("/usr/include/hailo")
        .compile("hailors");
    println!("cargo:rustc-link-lib=hailort");
}
