#include "wrapper.hpp"
#include <OpenImageIO/imageio.h>
#include <OpenImageIO/typedesc.h>
#include <algorithm>

const char* stdStringToCharArray(std::string str)
{
	char *chars = new char[str.size() + 1];
	strncpy(chars, str.c_str(), str.size() + 1);
	return chars;
}

OIIO::ImageSpec *COIIO_ImageSpec_new(OIIO::TypeDesc format) {
  return new OIIO::ImageSpec(format);
}

OIIO::ImageSpec *COIIO_ImageSpec_new_2D(int xres, int yres, int nchans,
                                        OIIO::TypeDesc fmt) {
  return new OIIO::ImageSpec(xres, yres, nchans, fmt);
}

void COIIO_ImageSpec_delete(OIIO::ImageSpec *ptr) { delete ptr; }

int COIIO_ImageSpec_x(const OIIO::ImageSpec *this_) { return this_->x; }
void COIIO_ImageSpec_set_x(OIIO::ImageSpec *this_, int x) { this_->x = x; }
int COIIO_ImageSpec_y(const OIIO::ImageSpec *this_) { return this_->y; }
void COIIO_ImageSpec_set_y(OIIO::ImageSpec *this_, int y) { this_->y = y; }
int COIIO_ImageSpec_z(const OIIO::ImageSpec *this_) { return this_->z; }
void COIIO_ImageSpec_set_z(OIIO::ImageSpec *this_, int z) { this_->z = z; }

int COIIO_ImageSpec_full_x(const OIIO::ImageSpec *this_) {
  return this_->full_x;
}
void COIIO_ImageSpec_set_full_x(OIIO::ImageSpec *this_, int x) {
  this_->full_x = x;
}
int COIIO_ImageSpec_full_y(const OIIO::ImageSpec *this_) {
  return this_->full_y;
}
void COIIO_ImageSpec_set_full_y(OIIO::ImageSpec *this_, int y) {
  this_->full_y = y;
}
int COIIO_ImageSpec_full_z(const OIIO::ImageSpec *this_) {
  return this_->full_z;
}
void COIIO_ImageSpec_set_full_z(OIIO::ImageSpec *this_, int z) {
  this_->full_z = z;
}

int COIIO_ImageSpec_width(const OIIO::ImageSpec *this_) { return this_->width; }
void COIIO_ImageSpec_set_width(OIIO::ImageSpec *this_, int width) {
  this_->width = width;
}
int COIIO_ImageSpec_height(const OIIO::ImageSpec *this_) {
  return this_->height;
}
void COIIO_ImageSpec_set_height(OIIO::ImageSpec *this_, int height) {
  this_->height = height;
}
int COIIO_ImageSpec_depth(const OIIO::ImageSpec *this_) { return this_->depth; }
void COIIO_ImageSpec_set_depth(OIIO::ImageSpec *this_, int depth) {
  this_->depth = depth;
}

int COIIO_ImageSpec_full_width(const OIIO::ImageSpec *this_) {
  return this_->full_width;
}
void COIIO_ImageSpec_set_full_width(OIIO::ImageSpec *this_, int width) {
  this_->full_width = width;
}
int COIIO_ImageSpec_full_height(const OIIO::ImageSpec *this_) {
  return this_->full_height;
}
void COIIO_ImageSpec_set_full_height(OIIO::ImageSpec *this_, int height) {
  this_->full_height = height;
}
int COIIO_ImageSpec_full_depth(const OIIO::ImageSpec *this_) {
  return this_->full_depth;
}
void COIIO_ImageSpec_set_full_depth(OIIO::ImageSpec *this_, int depth) {
  this_->full_depth = depth;
}

int COIIO_ImageSpec_tile_width(const OIIO::ImageSpec *this_) {
  return this_->tile_width;
}
void COIIO_ImageSpec_set_tile_width(OIIO::ImageSpec *this_, int width) {
  this_->tile_width = width;
}
int COIIO_ImageSpec_tile_height(const OIIO::ImageSpec *this_) {
  return this_->tile_height;
}
void COIIO_ImageSpec_set_tile_height(OIIO::ImageSpec *this_, int height) {
  this_->tile_height = height;
}
int COIIO_ImageSpec_tile_depth(const OIIO::ImageSpec *this_) {
  return this_->tile_depth;
}
void COIIO_ImageSpec_set_tile_depth(OIIO::ImageSpec *this_, int depth) {
  this_->tile_depth = depth;
}

int COIIO_ImageSpec_nchannels(const OIIO::ImageSpec *this_) {
  return this_->nchannels;
}
void COIIO_ImageSpec_set_nchannels(OIIO::ImageSpec *this_, int nchannels) {
  this_->nchannels = nchannels;
}

const OIIO::TypeDesc *COIIO_ImageSpec_format(const OIIO::ImageSpec *this_) {
  return &this_->format;
}
void COIIO_ImageSpec_set_format(OIIO::ImageSpec *this_,
                                OIIO::TypeDesc *format) {
  this_->format = *format;
}

// TypeDesc
const char *COIIO_TypeDesc_c_str(const OIIO::TypeDesc *this_) {
  return this_->c_str();
}
size_t COIIO_TypeDesc_fromstring(OIIO::TypeDesc *this_,
                                 const char *typestring) {
  return this_->fromstring(OIIO::string_view{typestring});
}

// ImageInput
OIIO::ImageInput *COIIO_ImageInput_open(const char *filename,
                                        const OIIO::ImageSpec *config) {
  return OIIO::ImageInput::open(filename, config);
}

void COIIO_ImageInput_close(OIIO::ImageInput *this_) { this_->close(); }

void COIIO_ImageInput_destroy(OIIO::ImageInput *input) {
  OIIO::ImageInput::destroy(input);
}

const OIIO::ImageSpec *COIIO_ImageInput_spec(const OIIO::ImageInput *this_) {
  return &this_->spec();
}

bool COIIO_ImageInput_read_image(OIIO::ImageInput *this_,
                                 const OIIO::TypeDesc *format, void *data) {
  return this_->read_image(*format, data);
}

const char *COIIO_ImageInput_geterror(OIIO::ImageInput *this_) {
	return stdStringToCharArray(this_->geterror());
}

const char *COIIO_geterror() {
  return stdStringToCharArray(OIIO::geterror());
}

/*int COIIO_geterror(char *buf, int bufsize) {
  auto msg = OIIO::geterror();
  if (buf)
    std::copy_n(msg.begin(), std::min(bufsize, (int)msg.size()), buf);
  return msg.size();
}*/

// ImageOutput
OIIO::ImageOutput *COIIO_ImageOutput_create(const char *filename,
                                            const char *plugin_searchpath) {
  return OIIO::ImageOutput::create(filename, plugin_searchpath);
}

bool COIIO_ImageOutput_open(OIIO::ImageOutput *this_, const char *filename,
                            const OIIO::ImageSpec *spec,
                            OIIO::ImageOutput::OpenMode mode) {
  return this_->open(filename, *spec, mode);
}

bool COIIO_ImageOutput_open_with_subimages(
    OIIO::ImageOutput *this_, const char *filename, int num_subimages,
    const OIIO::ImageSpec *subimage_specs) {
  return this_->open(filename, num_subimages, subimage_specs);
}

int COIIO_ImageOutput_supports(const OIIO::ImageOutput *this_,
                               const char *feature) {
  return this_->supports(feature);
}

bool COIIO_ImageOutput_write_image(OIIO::ImageOutput *this_,
                                   const OIIO::TypeDesc *format,
                                   const void *data, ptrdiff_t xstride,
                                   ptrdiff_t ystride, ptrdiff_t zstride) {
  return this_->write_image(*format, data, xstride, ystride, zstride);
}

bool COIIO_ImageOutput_close(OIIO::ImageOutput *this_) {
  return this_->close();
}

void COIIO_ImageOutput_destroy(OIIO::ImageOutput *imageout) {
  OIIO::ImageOutput::destroy(imageout);
}

const char *COIIO_ImageOutput_geterror(OIIO::ImageOutput *this_) {
	return stdStringToCharArray(this_->geterror());
}

void COIIO_delete_cstring(const char *ptr) { delete [] ptr; }