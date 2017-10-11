extern crate libc;
use std::os::raw::*;
use std::ptr;
use std::ffi::CString;

pub enum ImageSpec {}
pub enum ImageInput {}
pub enum ImageOutput {}

#[link(name="wrapper", kind="static")]
extern {
    pub fn COIIO_ImageSpec_new(format: *const TypeDesc) -> *mut ImageSpec;
    pub fn COIIO_ImageSpec_new_2D(xres: c_int, yres: c_int, nchans: c_int, fmt: *const TypeDesc) -> *mut ImageSpec;
    pub fn COIIO_ImageSpec_delete(ptr: *mut ImageSpec);

    pub fn COIIO_ImageSpec_x(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_x(this_: *mut ImageSpec, x: c_int);
    pub fn COIIO_ImageSpec_y(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_y(this_: *mut ImageSpec, y: c_int);
    pub fn COIIO_ImageSpec_z(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_z(this_: *mut ImageSpec, z: c_int);

    pub fn COIIO_ImageSpec_full_x(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_full_x(this_: *mut ImageSpec, x: c_int);
    pub fn COIIO_ImageSpec_full_y(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_full_y(this_: *mut ImageSpec, y: c_int);
    pub fn COIIO_ImageSpec_full_z(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_full_z(this_: *mut ImageSpec, z: c_int);

    pub fn COIIO_ImageSpec_width(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_width(this_: *mut ImageSpec, width: c_int);
    pub fn COIIO_ImageSpec_height(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_height(this_: *mut ImageSpec, height: c_int);
    pub fn COIIO_ImageSpec_depth(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_depth(this_: *mut ImageSpec, depth: c_int);

    pub fn COIIO_ImageSpec_full_width(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_full_width(this_: *mut ImageSpec, width: c_int);
    pub fn COIIO_ImageSpec_full_height(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_full_height(this_: *mut ImageSpec, height: c_int);
    pub fn COIIO_ImageSpec_full_depth(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_full_depth(this_: *mut ImageSpec, depth: c_int);

    pub fn COIIO_ImageSpec_tile_width(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_tile_width(this_: *mut ImageSpec, width: c_int);
    pub fn COIIO_ImageSpec_tile_height(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_tile_height(this_: *mut ImageSpec, height: c_int);
    pub fn COIIO_ImageSpec_tile_depth(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_tile_depth(this_: *mut ImageSpec, depth: c_int);

    pub fn COIIO_ImageSpec_nchannels(this_: *const ImageSpec) -> c_int;
    pub fn COIIO_ImageSpec_set_nchannels(this_: *mut ImageSpec, nchannels: c_int);
    pub fn COIIO_ImageSpec_format(this_: *const ImageSpec) -> *const TypeDesc;
    pub fn COIIO_ImageSpec_set_format(this_: *mut ImageSpec, format: *const TypeDesc);

    // TypeDesc
    pub fn COIIO_TypeDesc_c_str(this_: *const TypeDesc) -> *const c_char;
    pub fn COIIO_TypeDesc_fromstring(this: *mut TypeDesc, typestring: *const c_char);

    // ImageInput
    pub fn COIIO_ImageInput_open(filename: *const c_char, config: *const ImageSpec) -> *mut ImageInput;
    pub fn COIIO_ImageInput_close(this_: *mut ImageInput);
    pub fn COIIO_ImageInput_destroy(input: *mut ImageInput);
    pub fn COIIO_ImageInput_spec(this_: *const ImageInput) -> *const ImageSpec;
    pub fn COIIO_ImageInput_read_image(this_: *const ImageInput, format: TypeDesc, data: *mut c_void) -> bool;

    // Error handling
    pub fn COIIO_geterror(buf: *mut c_char, bufsize: c_int) -> c_int;

    // ImageOutput
    pub fn COIIO_ImageOutput_create(filename: *const c_char, plugin_searchpath: *const c_char) -> *mut ImageOutput;
    pub fn COIIO_ImageOutput_open(this_: *mut ImageOutput, filename: *const c_char, spec: *const ImageSpec, mode: ImageOutput_OpenMode) -> bool;
    pub fn COIIO_ImageOutput_open_with_subimages(this_: *mut ImageOutput, filename: *const c_char, num_subimages: c_int, subimage_specs: *const ImageSpec) -> bool;
    pub fn COIIO_ImageOutput_supports(this_: *const ImageOutput, feature: *const c_char) -> c_int;
    pub fn COIIO_ImageOutput_write_image(this_: *mut ImageOutput, format: *const TypeDesc, xstride: libc::ptrdiff_t, ystride: libc::ptrdiff_t, zstride: libc::ptrdiff_t) -> bool;
    pub fn COIIO_ImageOutput_close(this_: *mut ImageOutput) -> bool;
    pub fn COIIO_ImageOutput_destroy(imageout: *mut ImageOutput);
}

#[repr(C)]
pub enum ImageOutput_OpenMode
{
    Create = 0,
    AppendSubimage,
    AppendMIPLevel
}

#[repr(u8)]
pub enum Basetype {
    Unknown,
    None,
    UInt8,
    Int8,
    UInt16,
    Int16,
    UInt32,
    Int32,
    UInt64,
    Int64,
    Half,
    Float,
    Double,
    String,
    Ptr,
    LastBase
}

#[repr(u8)]
pub enum AggregateKind {
    Scalar=1,
    Vec2=2,
    Vec3=3,
    Vec4=4,
    Matrix33=9,
    Matrix44=16
}

#[repr(u8)]
pub enum VecSemantics {
    NoSemantics=0,  // no semantic hints
    Color,    // color
    Point,    // spatial location
    Vector,   // spatial direction
    Normal,   // surface normal
    Timecode, // SMPTE timecode (should be int[2])
    Keycode   // SMPTE keycode (should be int[7])
}

#[repr(C)]
pub struct TypeDesc {
    basetype: Basetype,
    aggregate: AggregateKind,
    vecsemantics: VecSemantics,
    reserved: c_uchar,
    arraylen: c_int,
}

impl TypeDesc {
    pub fn from_basetype(basetype: Basetype, aggregate: AggregateKind, vecsemantics: VecSemantics) -> TypeDesc {
        TypeDesc {
            aggregate,
            basetype,
            arraylen: 1,
            reserved: 0,
            vecsemantics
        }
    }
}


#[test]
fn test_open_image()
{
    unsafe {
        println!("Loading image...");
        let filepath = CString::new("../test_images/tonberry.jpg").unwrap();
        let img = COIIO_ImageInput_open(filepath.as_ptr(), ptr::null());
        assert_ne!(img, ptr::null_mut());
        let spec = COIIO_ImageInput_spec(img);
        let width = COIIO_ImageSpec_width(spec);
        let height = COIIO_ImageSpec_height(spec);
        println!("width={}, height={}", width, height);
        COIIO_ImageInput_close(img);
    }
}
