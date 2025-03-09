extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    //
    // Link to `libdemo` dynamic library file
    //
    println!("cargo:rustc-link-lib=dylib=sample");
    println!("cargo:rustc-link-search=native=../build/sample/");
    println!("cargo:rerun-if-changed=../sample/inc/myclass_ns.h");
    println!("cargo:rerun-if-changed=../sample/inc/myclass.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("out_put: {:#?}", &out_path);
    let bindings = bindgen::Builder::default()
        .clang_args(&[
            "-include",
            "../sample/inc/myclass.h",
            "-include",
            "../sample/inc/myclass_ns.h",
        ])
        .header("../sample/inc/myclass_inherit.h")
        // Enable C++ namespace support
        .enable_cxx_namespaces()
        // Add extra clang args for supporting `C++`
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        //        .clang_arg("-stdlib=libc++")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // deprecated at 0.71
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
