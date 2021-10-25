extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=hell");
    println!("cargo:rustc-link-lib=dylib=winmm");
    println!("cargo:rustc-link-lib=dylib=secur32");
    println!("cargo:rustc-link-lib=dylib=dmoguids");
    println!("cargo:rustc-link-lib=dylib=wmcodecdspuuid");
    println!("cargo:rustc-link-lib=dylib=amstrmid");
    println!("cargo:rustc-link-lib=dylib=msdmo");
    println!("cargo:rustc-link-lib=dylib=gdi32");
    println!("cargo:rustc-link-lib=dylib=d3d11");
    println!("cargo:rustc-link-lib=dylib=dxgi");

    println!("cargo:rerun-if-changed=include/hell.hpp");

    let bindings = bindgen::Builder::default()
        .header("include/hell.hpp")
        .clang_arg("--include-directory=include")
        .clang_arg("--include-directory=include/third_party/abseil-cpp")
        .clang_arg("--include-directory=include/third_party/googletest/src/googletest/include")
        .clang_arg("--include-directory=include/third_party/googletest/src/googlemock/include")
        .clang_arg("-DWEBRTC_WIN")
        .clang_arg("-DNOMINMAX")
        .clang_arg("-DWEBRTC_USE_BUILTIN_ISAC_FLOAT")
        .clang_arg("--")
        .clang_arg("-std=c++11")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}