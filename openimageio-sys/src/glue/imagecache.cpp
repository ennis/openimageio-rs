#include <OpenImageIO/imagebuf.h>
#include "oiio.h"
#include "helpers.hpp"
#include <string>

extern "C" {


OIIO_ImageCache *OIIO_ImageCache_create(bool shared) {
    return (OIIO_ImageCache *) OIIO::ImageCache::create(shared);
}

void OIIO_ImageCache_destroy(OIIO_ImageCache *x, bool teardown) {
    OIIO::ImageCache::destroy(OIIO_CAST(ImageCache, x), teardown);
}

void OIIO_ImageCache_clear(OIIO_ImageCache *x) {
    OIIO_CAST(ImageCache, x)->clear();
}

const char *OIIO_ImageCache_geterror(const OIIO_ImageCache *x) {
    std::string str = OIIO_CAST_CONST(ImageCache, x)->geterror();
    return makeCString(str);
}

const char *OIIO_ImageCache_getstats(const OIIO_ImageCache *x, int level) {
    std::string str = OIIO_CAST_CONST(ImageCache, x)->getstats(level);
    return makeCString(str);
}

void OIIO_ImageCache_reset_stats(OIIO_ImageCache *x) {
    OIIO_CAST(ImageCache, x)->reset_stats();
}

void OIIO_ImageCache_invalidate(OIIO_ImageCache *x, OIIO_StringRef filename) {
    OIIO::ustring s{filename.ptr, filename.len};
    OIIO_CAST(ImageCache, x)->invalidate(s);
}

void OIIO_ImageCache_invalidate_all(OIIO_ImageCache *x, bool force) {
    OIIO_CAST(ImageCache, x)->invalidate_all(force);
}

bool OIIO_ImageCache_attribute(OIIO_ImageCache *x, OIIO_StringRef name, OIIO_TypeDesc type, const void *val) {
    return OIIO_CAST(ImageCache, x)->attribute(OIIO::string_view{name.ptr, name.len}, unwrapTypeDesc(type), val);
}

bool OIIO_ImageCache_getattribute(OIIO_ImageCache *x, OIIO_StringRef name, OIIO_TypeDesc type, void *val) {
    return OIIO_CAST(ImageCache, x)->getattribute(OIIO::string_view{name.ptr, name.len}, unwrapTypeDesc(type), val);
}

OIIO_ImageCache_Perthread *
OIIO_ImageCache_get_perthread_info(OIIO_ImageCache *x, OIIO_ImageCache_Perthread *thread_info) {
    return (OIIO_ImageCache_Perthread *) OIIO_CAST(ImageCache, x)->get_perthread_info(
            OIIO_CAST(ImageCache::Perthread, thread_info));
}

OIIO_ImageCache_Perthread *OIIO_ImageCache_create_perthread_info(OIIO_ImageCache *x) {
    return (OIIO_ImageCache_Perthread *) OIIO_CAST(ImageCache, x)->create_thread_info();

}

void OIIO_ImageCache_destroy_perthread_info(OIIO_ImageCache *x, OIIO_ImageCache_Perthread *thread_info) {
    OIIO_CAST(ImageCache, x)->destroy_thread_info(OIIO_CAST(ImageCache::Perthread, thread_info));
}

OIIO_ImageCache_ImageHandle *OIIO_ImageCache_get_image_handle(OIIO_ImageCache *x, OIIO_StringRef name) {
    // Temporary fix for OIIO bug if Perthread is NULL
    auto thread_info = OIIO_CAST(ImageCache, x)->get_perthread_info();
    return (OIIO_ImageCache_ImageHandle *) OIIO_CAST(ImageCache, x)->get_image_handle(OIIO::ustring{name.ptr, name.len},
                                                                                      thread_info);
}

bool OIIO_ImageCache_good(OIIO_ImageCache *x, OIIO_ImageCache_ImageHandle *file) {
    return OIIO_CAST(ImageCache, x)->good(OIIO_CAST(ImageCache::ImageHandle, file));
}

bool OIIO_ImageCache_get_image_info(OIIO_ImageCache *x, OIIO_StringRef filename, int subimage, int miplevel,
                                    OIIO_StringRef dataname, OIIO_TypeDesc datatype, void *data) {
    return OIIO_CAST(ImageCache, x)->get_image_info(
            OIIO::ustring{filename.ptr, filename.len},
            subimage, miplevel,
            OIIO::ustring{dataname.ptr, dataname.len},
            unwrapTypeDesc(datatype),
            data);
}


bool OIIO_ImageCache_get_image_info_by_handle(OIIO_ImageCache *x, OIIO_ImageCache_ImageHandle *file,
                                              OIIO_ImageCache_Perthread *thread_info, int subimage, int miplevel,
                                              OIIO_StringRef dataname, OIIO_TypeDesc datatype, void *data) {
    return OIIO_CAST(ImageCache, x)->get_image_info(
            OIIO_CAST(ImageCache::ImageHandle, file),
            OIIO_CAST(ImageCache::Perthread, thread_info),
            subimage, miplevel,
            OIIO::ustring{dataname.ptr, dataname.len},
            unwrapTypeDesc(datatype),
            data);
}

bool OIIO_ImageCache_get_imagespec(OIIO_ImageCache *x, OIIO_StringRef filename, OIIO_ImageSpec *spec,
                                   int subimage, int miplevel, bool native) {
    return OIIO_CAST(ImageCache, x)->get_imagespec(
            OIIO::ustring{filename.ptr, filename.len},
            *OIIO_CAST(ImageSpec, spec),
            subimage, miplevel,
            native);
}

bool OIIO_ImageCache_get_imagespec_by_handle(OIIO_ImageCache *x, OIIO_ImageCache_ImageHandle *file,
                                             OIIO_ImageCache_Perthread *thread_info,
                                             OIIO_ImageSpec *spec, int subimage, int miplevel, bool native) {
    // Temporary fix for OIIO bug if Perthread is NULL
    if (!thread_info) {
        thread_info = (OIIO_ImageCache_Perthread*)OIIO_CAST(ImageCache, x)->get_perthread_info();
    }

    return OIIO_CAST(ImageCache, x)->get_imagespec(
            OIIO_CAST(ImageCache::ImageHandle, file),
            OIIO_CAST(ImageCache::Perthread, thread_info),
            *OIIO_CAST(ImageSpec, spec),
            subimage, miplevel,
            native);
}

const char *OIIO_ImageCache_resolve_filename(OIIO_ImageCache *x, OIIO_StringRef filename) {
    auto str = OIIO_CAST(ImageCache, x)->resolve_filename(std::string{filename.ptr, filename.len});
    return makeCString(str);
}

bool OIIO_ImageCache_get_pixels(OIIO_ImageCache *x,
                                OIIO_StringRef filename,
                                int subimage, int miplevel,
                                int xbegin, int xend,
                                int ybegin, int yend,
                                int zbegin, int zend,
                                OIIO_TypeDesc format, void *result) {
    return OIIO_CAST(ImageCache, x)->get_pixels(OIIO::ustring{filename.ptr, filename.len}, subimage, miplevel, xbegin,
                                                xend, ybegin, yend, zbegin, zend, unwrapTypeDesc(format), result);
}

bool OIIO_ImageCache_get_pixels_by_handle(OIIO_ImageCache *x,
                                 OIIO_ImageCache_ImageHandle *file,
                                 OIIO_ImageCache_Perthread *thread_info,
                                 int subimage, int miplevel,
                                 int xbegin, int xend,
                                 int ybegin, int yend,
                                 int zbegin, int zend,
                                 OIIO_TypeDesc format, void *result) {

    return OIIO_CAST(ImageCache, x)->get_pixels(OIIO_CAST(ImageCache::ImageHandle, file),
                                                OIIO_CAST(ImageCache::Perthread, thread_info), subimage, miplevel,
                                                xbegin, xend, ybegin, yend, zbegin, zend, unwrapTypeDesc(format),
                                                result);
}

bool OIIO_ImageCache_get_pixels_stride(OIIO_ImageCache *x,
                                         OIIO_StringRef filename, int subimage, int miplevel,
                                         int xbegin, int xend, int ybegin, int yend,
                                         int zbegin, int zend, int chbegin, int chend,
                                         OIIO_TypeDesc format, void *result,
                                         stride_t xstride, stride_t ystride, stride_t zstride,
                                         int cache_chbegin, int cache_chend) {
    return OIIO_CAST(ImageCache, x)->get_pixels(OIIO::ustring{filename.ptr, filename.len},
                                                subimage, miplevel,
                                                xbegin, xend, ybegin, yend,
                                                zbegin, zend, chbegin, chend,
                                                unwrapTypeDesc(format), result,
                                                xstride, ystride, zstride,
                                                cache_chbegin, cache_chend);
}

bool OIIO_ImageCache_get_pixels_stride_by_handle(OIIO_ImageCache *x, OIIO_ImageCache_ImageHandle *file,
                                          OIIO_ImageCache_Perthread *thread_info,
                                          int subimage, int miplevel,
                                          int xbegin, int xend, int ybegin, int yend,
                                          int zbegin, int zend, int chbegin, int chend,
                                          OIIO_TypeDesc format, void *result,
                                          stride_t xstride, stride_t ystride, stride_t zstride,
                                          int cache_chbegin, int cache_chend) {
    return OIIO_CAST(ImageCache, x)->get_pixels(OIIO_CAST(ImageCache::ImageHandle, file),
                                                OIIO_CAST(ImageCache::Perthread, thread_info), subimage, miplevel,
                                                xbegin, xend, ybegin, yend,
                                                zbegin, zend, chbegin, chend,
                                                unwrapTypeDesc(format), result,
                                                xstride, ystride, zstride,
                                                cache_chbegin, cache_chend);
}


}



