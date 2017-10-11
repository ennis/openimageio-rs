extern crate vcpkg;
extern crate pkg_config;
extern crate bindgen;
extern crate gcc;

use std::error::Error;
use std::env;
use std::path::PathBuf;

fn main()
{
    let mut found_oiio = false;
    let mut include_paths = Vec::new();
    env::set_var("VCPKGRS_DYNAMIC", "1");
    // try vcpkg on windows
    eprintln!("trying to detect/fetch OpenImageIO through vcpkg...");

    let lib = vcpkg::Config::new().copy_dlls(true).probe("openimageio");
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
        },
        Err(err) => {
            eprintln!("ERROR: {}", err.description());
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
                found_oiio = true;
                include_paths = lib.include_paths;
            },
            Err(err) => {
                eprintln!("ERROR: {}", err.description());
            }
        }
    }

    // nothing worked, bail out
    if !found_oiio {
        panic!("Could not find OpenImageIO");
    }

    // bindgen

    /*let bindings= {
        let mut builder = bindgen::Builder::default();
        // The input header we would like to generate
        // bindings for.
        builder = builder.header("bindings/wrapper.hpp")
            .clang_arg("-std=c++14")
            .clang_arg("-v")
            .disable_name_namespacing()
            // don't include methods and functions: we replace them with a C API
            .ignore_methods()
            // Hide std (shouldn't appear anyway)
            .hide_type("std::.*")
            // whitelist all easily representable types
            .whitelisted_type(".*TypeDesc")
            // make all types containing std stuff opaque
            .opaque_type("*.ImageSpec")
            .opaque_type("*.ImageInput")
            .opaque_type("*.ImageOutput")
            // whitelist our C API
            .whitelisted_function("COIIO_.*");

        // add include paths
        for p in include_paths {
            builder = builder.clang_arg(format!("-I{}", p.to_str().unwrap()));
            //println!("-I{}", p.to_str().unwrap());
        }
        // Finish the builder and generate the bindings.
        builder.generate()
    }
        // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("oiio_bindings.rs"))
        .expect("Couldn't write bindings!");*/

    // compile bindings
    let mut build = gcc::Build::new();
    build.file("wrapper/wrapper.cpp");
    build.include("wrapper");
    for p in include_paths {
        build.include(p);
    }
    build.compile("wrapper");
}