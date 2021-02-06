#include <OpenImageIO/imageio.h>

#include "helpers.hpp"
#include "oiio.h"
#include <string>

extern "C" {

void OIIO_ImageOutput_delete(OIIO_ImageOutput *out) { delete OIIO_RS_CAST(ImageOutput, out); }

bool OIIO_ImageOutput_open(OIIO_ImageOutput *out, OIIO_StringRef name,
                           const OIIO_ImageSpec *newspec, OIIO_ImageOutput_OpenMode openmode) {
  std::string s_filename{name.ptr, name.len};
  return OIIO_RS_CAST(ImageOutput, out)
      ->open(s_filename, *OIIO_RS_CAST_CONST(ImageSpec, newspec),
             static_cast<OIIO::ImageOutput::OpenMode>(openmode));
}

bool OIIO_ImageOutput_close(OIIO_ImageOutput *out) {
  return OIIO_RS_CAST(ImageOutput, out)->close();
}

bool OIIO_ImageOutput_write_image(OIIO_ImageOutput *out, OIIO_TypeDesc format, const void *data,
                                  ptrdiff_t xstride, ptrdiff_t ystride, ptrdiff_t zstride) {
  return OIIO_RS_CAST(ImageOutput, out)
      ->write_image(helpers::unwrapTypeDesc(format), data, xstride, ystride, zstride);
}

bool OIIO_ImageOutput_write_scanline(OIIO_ImageOutput *out, int y, int z, OIIO_TypeDesc format,
                                     const void *data, stride_t xstride) {
  return OIIO_RS_CAST(ImageOutput, out)
      ->write_scanline(y, z, helpers::unwrapTypeDesc(format), data, xstride);
}

bool OIIO_ImageOutput_open2(OIIO_ImageOutput *out, OIIO_StringRef name, int subimages,
                            const OIIO_ImageSpec *const *specs) {
  std::string s_filename{name.ptr, name.len};
  // copy because of unfortunate API decisions
  std::vector<OIIO::ImageSpec> specs_copy;
  specs_copy.reserve(subimages);
  for (int i = 0; i < subimages; ++i) {
    specs_copy.push_back(*OIIO_RS_CAST_CONST(ImageSpec, specs[i]));
  }
  return OIIO_RS_CAST(ImageOutput, out)->open(s_filename, subimages, specs_copy.data());
}

OIIO_ImageOutput *OIIO_ImageOutput_create(OIIO_StringRef filename,
                                          OIIO_StringRef plugin_searchpath) {
  std::string s_filename{filename.ptr, filename.len};
  std::string s_path{plugin_searchpath.ptr, plugin_searchpath.len};
  return (OIIO_ImageOutput *)OIIO::ImageOutput::create(s_filename, s_path).release();
}

const char *OIIO_ImageOutput_geterror(const OIIO_ImageOutput *out) {
  std::string sstring = OIIO_RS_CAST_CONST(ImageOutput, out)->geterror();
  return helpers::makeCString(sstring);
}

const char *OIIO_ImageOutput_format_name(const OIIO_ImageOutput *out) {
  return OIIO_RS_CAST_CONST(ImageOutput, out)->format_name();
}

const OIIO_ImageSpec *OIIO_ImageOutput_spec(const OIIO_ImageOutput *out) {
  return (OIIO_ImageSpec *)(&(OIIO_RS_CAST_CONST(ImageOutput, out)->spec()));
}

bool OIIO_ImageOutput_supports(const OIIO_ImageOutput *out, OIIO_StringRef feature) {
  OIIO::string_view s_feature{feature.ptr, feature.len};
  return OIIO_RS_CAST_CONST(ImageOutput, out)->supports(s_feature);
}

} // extern "C"
