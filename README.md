# openimageio-rs

(Experimental) Rust bindings to the [OpenImageIO](https://github.com/OpenImageIO/oiio) image loading library. 

### Prerequisites

openimageio-rs does not currently attempt to compile the native C++ library from scratch. 
Instead, the native C++ library must already be installed on your system. 
The build script will try to find it via `pkg-config` on 
Linux, or `vcpkg-rs` on Windows. 
Currently, only OpenImageIO **1.8** is supported.

Use the following vcpkg command to install the native library on Windows:
```
vcpkg install openimageio
```



### Usage

Add the following line to the `[dependencies]` in `Cargo.toml`:
```toml
openimageio = { git = "https://github.com/ennis/openimageio-rs" }
```

#### Example usages

TODO fill this section

