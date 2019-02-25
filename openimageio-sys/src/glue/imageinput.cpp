#include <OpenImageIO/imageio.h>

#include <string>

#include "oiio.h"
#include "helpers.hpp"

extern "C" {

void OIIO_ImageInput_delete(OIIO_ImageInput *in) {
    delete OIIO_CAST(ImageInput ,in);
}

OIIO_ImageInput *OIIO_ImageInput_open(OIIO_StringRef filename, const OIIO_ImageSpec *config) {
    std::string s_filename{filename.ptr, filename.len};
    return (OIIO_ImageInput *)OIIO::ImageInput::open(s_filename, OIIO_CAST_CONST(ImageSpec ,config));
}

OIIO_ImageInput *OIIO_ImageInput_create(OIIO_StringRef filename, OIIO_StringRef plugin_searchpath) {
    std::string s_filename{filename.ptr, filename.len};
    std::string s_path{plugin_searchpath.ptr, plugin_searchpath.len};
    return (OIIO_ImageInput *)OIIO::ImageInput::create(s_filename, s_path);
}

const char *OIIO_ImageInput_geterror(const OIIO_ImageInput *in) {
    std::string sstring = OIIO_CAST_CONST(ImageInput ,in)->geterror();
    return makeCString(sstring);
}

const char *OIIO_ImageInput_format_name(OIIO_ImageInput *in) {
    return OIIO_CAST(ImageInput ,in)->format_name();
}

bool OIIO_ImageInput_valid_file(OIIO_ImageInput *in, OIIO_StringRef filename) {
    std::string s_name{filename.ptr, filename.len};
    return OIIO_CAST(ImageInput ,in)->valid_file(s_name);
}

bool OIIO_ImageInput_open2(OIIO_ImageInput *in, OIIO_StringRef name, OIIO_ImageSpec *newspec) {
    std::string s_name{name.ptr, name.len};
    return OIIO_CAST(ImageInput ,in)->open(s_name, *OIIO_CAST(ImageSpec ,newspec));
}

const OIIO_ImageSpec *OIIO_ImageInput_spec(const OIIO_ImageInput *in) {
    const OIIO::ImageSpec *spec = &(OIIO_CAST_CONST(ImageInput ,in)->spec());
    return (const OIIO_ImageSpec *)spec;
}

bool OIIO_ImageInput_supports(const OIIO_ImageInput *in, OIIO_StringRef feature) {
    OIIO::string_view s_feature{feature.ptr, feature.len};
    return OIIO_CAST_CONST(ImageInput ,in)->supports(s_feature);
}

bool OIIO_ImageInput_close(OIIO_ImageInput *in) {
    return OIIO_CAST(ImageInput ,in)->close();
}

int OIIO_ImageInput_current_subimage(const OIIO_ImageInput *in) {
    return OIIO_CAST_CONST(ImageInput ,in)->current_subimage();
}

bool OIIO_ImageInput_seek_subimage(OIIO_ImageInput *in, int subimage, OIIO_ImageSpec *newspec) {
    return OIIO_CAST(ImageInput ,in)->seek_subimage(
            subimage,
            *OIIO_CAST(ImageSpec ,newspec));
}

int OIIO_ImageInput_current_miplevel(const OIIO_ImageInput *in) {
    return OIIO_CAST_CONST(ImageInput ,in)->current_miplevel();
}

bool OIIO_ImageInput_seek_subimage_miplevel(OIIO_ImageInput *in, int subimage, int miplevel, OIIO_ImageSpec *newspec) {
    return OIIO_CAST(ImageInput ,in)->seek_subimage(
            subimage,
            miplevel,
            *OIIO_CAST(ImageSpec ,newspec));
}

bool OIIO_ImageInput_read_image_floats(OIIO_ImageInput *in, float *data) {
    return OIIO_CAST(ImageInput ,in)->read_image(data);
}

bool OIIO_ImageInput_read_image_format(OIIO_ImageInput *in, OIIO_TypeDesc format, void *data, void *cbk_data) {
    ProgressCallback cbk = nullptr;
    /*if (cbk_data != nullptr) {
        cbk = &image_progress_callback;
    }*/

    return OIIO_CAST(ImageInput ,in)->read_image(unwrapTypeDesc(format),
                                                                data,
                                                                OIIO::AutoStride,
                                                                OIIO::AutoStride,
                                                                OIIO::AutoStride,
                                                                cbk,
                                                                cbk_data);
}


bool OIIO_ImageInput_read_image_format2(
        OIIO_ImageInput *in,
        int chbegin,
        int chend,
        OIIO_TypeDesc format,
        void *data,
        stride_t xstride,
        stride_t ystride,
        stride_t zstride,
        void *cbk_data) {
    ProgressCallback cbk = nullptr;

    return OIIO_CAST(ImageInput ,in)->read_image(
            chbegin, chend,
            unwrapTypeDesc(format),
            data,
            xstride,
            ystride,
            zstride,
            cbk,
            cbk_data);
}

bool OIIO_ImageInput_read_scanline_floats(OIIO_ImageInput *in, int y, int z, float *data) {
    return OIIO_CAST(ImageInput ,in)->read_scanline(y, z, data);
}

bool OIIO_ImageInput_read_tile_floats(OIIO_ImageInput *in, int x, int y, int z, float *data) {
    return OIIO_CAST(ImageInput ,in)->read_tile(x, y, z, data);
}


} // extern "C"


