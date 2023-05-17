extern crate bindgen;
extern crate cc;
extern crate pkg_config;
extern crate vcpkg;

use std::{env, path::PathBuf};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let mut found_oiio = false;
    let mut include_paths = Vec::new();

    eprintln!("trying to detect/fetch OpenImageIO via vcpkg...");

    let lib = vcpkg::find_package("openimageio");
    match lib {
        Ok(lib) => {
            eprintln!("found OpenImageIO through vcpkg");
            eprintln!("-> libs: {:?}", lib.found_libs);
            eprintln!("-> link paths: {:?}", lib.link_paths);
            eprintln!("-> include paths: {:?}", lib.include_paths);
            eprintln!("-> DLLs: {:?}", lib.found_dlls);
            eprintln!("-> DLL paths: {:?}", lib.dll_paths);
            found_oiio = true;
            include_paths = lib.include_paths;
        }
        Err(err) => {
            eprintln!("ERROR: {}", err);
        }
    }

    if !found_oiio {
        // try pkg-config
        eprintln!("-> trying to detect OpenImageIO through pkg-config...");
        let lib = pkg_config::probe_library("openimageio");
        match lib {
            Ok(lib) => {
                eprintln!("Found OpenImageIO through pkg-config");
                eprintln!("-> libs: {:?}", lib.libs);
                eprintln!("-> link paths: {:?}", lib.link_paths);
                eprintln!("-> include paths: {:?}", lib.include_paths);
                eprintln!("-> defines: {:?}", lib.defines);
                found_oiio = true;
                include_paths = lib.include_paths;
            }
            Err(err) => {
                eprintln!("ERROR: {}", err);
            }
        }
    }

    // nothing worked, bail out
    if !found_oiio {
        panic!("Could not find OpenImageIO");
    }

    // bindgen our functions
    let bindings = {
        let mut builder = bindgen::Builder::default();
        // The input header we would like to generate
        builder = builder
            .header("src/glue/oiio.h")
            .clang_arg("-v")
            .derive_copy(true)
            .derive_eq(true)
            .size_t_is_usize(true)
            .with_codegen_config(
                bindgen::CodegenConfig::TYPES
                    | bindgen::CodegenConfig::FUNCTIONS
                    | bindgen::CodegenConfig::VARS,
            )
            .prepend_enum_name(false);

        // add include paths
        for p in include_paths.iter() {
            builder = builder.clang_arg(format!("-I{}", p.to_str().unwrap()));
            println!("-I{}", p.to_str().unwrap());
        }
        // Finish the builder and generate the bindings.
        builder.generate()
    }
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // compile bindings
    let mut build = cc::Build::new();
    //build.file("src/glue/color.cpp");
    //build.file("src/glue/imagebuf.cpp");
    //build.file("src/glue/imagebufalgo.cpp");
    //build.file("src/glue/imagecache.cpp");
    build.file("src/glue/helpers.cpp");
    build.file("src/glue/imageinput.cpp");
    build.file("src/glue/imageoutput.cpp");
    build.file("src/glue/imagespec.cpp");
    build.file("src/glue/imagecache.cpp");
    build.file("src/glue/oiio.cpp");
    //build.file("src/glue/roi.cpp");
    build.include("src/glue");
    for p in include_paths {
        build.include(p);
    }
    // fix for OpenImageIO: usage of CommandLineToArgvW requires shell32
    if target_os == "windows" {
        println!("cargo:rustc-link-lib=shell32")
    }
    build.define("OIIO_STATIC_DEFINE", None);
    build.cpp(true);
    //if build.get_compiler().is_like_msvc() {
    //    build.flag("/std:c++17");
    //} else {
        build.flag("-std=c++17");
    //}
    build.compile("glue");
}
