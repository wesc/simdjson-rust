use bindgen;
use cmake::Config;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if !Path::new("simdjson/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status()
            .unwrap();
    }

    let mut dst = Config::new("simdjson")
        // .define("SIMDJSON_BUILD_STATIC", "ON")    
        .build();
    // dst.push("build");
    // dst.push("src");
    let mut dst = cmake::build("simdjson");
    dst.push("lib64");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=simdjson");
    println!("cargo:rustc-link-lib=stdc++");

    let mut header_path = PathBuf::from("simdjson");
    header_path.push("singleheader");
    header_path.push("simdjson.h");
    // header_path.push("include");
    // header_path.push("simdjson");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header_path.to_string_lossy())
        // .header_contents("wrapper.h","#include<jsonparser.h>")
        // .clang_arg(format!("-L{}", header_path.display()))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        // .clang_arg("-lc++abi")
        // .clang_arg("-lstdc++")
        // .clang_arg("-static")
        // .clang_arg("-stdlib=libc++")
        .whitelist_function("json_parse")
        .whitelist_function("build_parsed_json")
        .opaque_type("std::.*")
        // .whitelist_type("ParsedJson")
        // .opaque_type("ParsedJson")
        // .blacklist_type("std::.*")
        // .blacklist_function("dump")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}