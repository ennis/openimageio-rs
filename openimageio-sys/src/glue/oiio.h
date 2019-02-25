#ifndef _OPENIMAGEIO_H_
#define _OPENIMAGEIO_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    unsigned char basetype;     ///< C data type at the heart of our type
    unsigned char aggregate;    ///< What kind of AGGREGATE is it?
    unsigned char vecsemantics; ///< What does the vec represent?
    unsigned char reserved;     ///< Reserved for future expansion
    int arraylen;               ///< Array length, 0 = not array, -1 = unsized
} OIIO_TypeDesc;

typedef enum {
    OIIO_TypeDesc_BaseType_Unknown,
    OIIO_TypeDesc_BaseType_None,
    OIIO_TypeDesc_BaseType_UInt8,
    OIIO_TypeDesc_BaseType_Int8,
    OIIO_TypeDesc_BaseType_UInt16,
    OIIO_TypeDesc_BaseType_Int16,
    OIIO_TypeDesc_BaseType_UInt32,
    OIIO_TypeDesc_BaseType_Int32,
    OIIO_TypeDesc_BaseType_UInt64,
    OIIO_TypeDesc_BaseType_Int64,
    OIIO_TypeDesc_BaseType_Half,
    OIIO_TypeDesc_BaseType_Float,
    OIIO_TypeDesc_BaseType_Double,
    OIIO_TypeDesc_BaseType_String,
    OIIO_TypeDesc_BaseType_Ptr,
    OIIO_TypeDesc_BaseType_Lastbase
} OIIO_TypeDesc_BaseType;

typedef enum {
    OIIO_TypeDesc_Aggregate_Scalar = 1,
    OIIO_TypeDesc_Aggregate_Vec2 = 2,
    OIIO_TypeDesc_Aggregate_Vec3 = 3,
    OIIO_TypeDesc_Aggregate_Vec4 = 4,
    OIIO_TypeDesc_Aggregate_Matrix33 = 9,
    OIIO_TypeDesc_Aggregate_Matrix44 = 16
} OIIO_TypeDesc_Aggregate;

typedef enum {
    OIIO_TypeDesc_VecSemantics_NoSemantics,  // no semantic hints
    OIIO_TypeDesc_VecSemantics_Color,    // color
    OIIO_TypeDesc_VecSemantics_Point,    // spatial location
    OIIO_TypeDesc_VecSemantics_Vector,   // spatial direction
    OIIO_TypeDesc_VecSemantics_Normal,   // surface normal
    OIIO_TypeDesc_VecSemantics_Timecode, // SMPTE timecode (should be int[2])
    OIIO_TypeDesc_VecSemantics_Keycode,  // SMPTE keycode (should be int[7])
    OIIO_TypeDesc_VecSemantics_Rational  // paired numerator and denominator
} OIIO_TypeDesc_VecSemantics;

typedef ptrdiff_t stride_t;
static const stride_t OIIO_AutoStride = PTRDIFF_MIN;
typedef unsigned long long imagesize_t;

typedef struct OIIO_ImageSpec OIIO_ImageSpec;
typedef struct OIIO_ImageInput OIIO_ImageInput;
typedef struct OIIO_ImageOutput OIIO_ImageOutput;
typedef struct OIIO_ImageCache OIIO_ImageCache;
typedef struct OIIO_ImageCache_ImageHandle OIIO_ImageCache_ImageHandle;
typedef struct OIIO_ImageCache_Perthread OIIO_ImageCache_Perthread;


typedef enum {
    OIIO_ImageOutput_OpenMode_Create,
    OIIO_ImageOutput_OpenMode_AppendSubimage,
    OIIO_ImageOutput_OpenMode_AppendMIPLevel,
} OIIO_ImageOutput_OpenMode;

typedef bool(*ProgressCallback)(void *opaque_data, float portion_done);

typedef struct {
    const char *ptr;
    size_t len;
} OIIO_StringRef;

void OIIO_freeString(const char *ptr);
const char *OIIO_geterror();

//---------------------------------------------------------------------
// OIIO_ImageInput

void OIIO_ImageInput_delete(OIIO_ImageInput *in);
OIIO_ImageInput *OIIO_ImageInput_open(OIIO_StringRef filename, const OIIO_ImageSpec *config);
OIIO_ImageInput *OIIO_ImageInput_create(OIIO_StringRef filename, OIIO_StringRef plugin_searchpath);

const char *OIIO_ImageInput_format_name(OIIO_ImageInput *in);
bool OIIO_ImageInput_valid_file(OIIO_ImageInput *in, OIIO_StringRef filename);
bool OIIO_ImageInput_open2(OIIO_ImageInput *in, OIIO_StringRef name, OIIO_ImageSpec *newspec);
const OIIO_ImageSpec *OIIO_ImageInput_spec(const OIIO_ImageInput *in);
bool OIIO_ImageInput_supports(const OIIO_ImageInput *in, OIIO_StringRef feature);
bool OIIO_ImageInput_close(OIIO_ImageInput *in);
int OIIO_ImageInput_current_subimage(const OIIO_ImageInput *in);
int OIIO_ImageInput_current_miplevel(const OIIO_ImageInput *in);
bool OIIO_ImageInput_seek_subimage(OIIO_ImageInput *in, int subimage, OIIO_ImageSpec *newspec);
bool OIIO_ImageInput_seek_subimage_miplevel(OIIO_ImageInput *in, int subimage, int miplevel, OIIO_ImageSpec *newspec);
bool OIIO_ImageInput_read_scanline_floats(OIIO_ImageInput *in, int y, int z, float *data);
// bool OIIO_ImageInput_read_scanline_format(OIIO_ImageInput *in, int y, int z, OIIO_TypeDesc format, void* data, stride_t xstride);
bool OIIO_ImageInput_read_tile_floats(OIIO_ImageInput *in, int x, int y, int z, float *data);
// bool OIIO_ImageInput_read_tile_format(OIIO_ImageInput *in, int x, int y, int z, OIIO_TypeDesc format, void* data,
// 									stride_t xstride, stride_t ystride, stride_t zstride);
bool OIIO_ImageInput_read_image_floats(OIIO_ImageInput *in, float *data);
bool OIIO_ImageInput_read_image_format(OIIO_ImageInput *in, OIIO_TypeDesc format, void *data, void *cbk_data);
bool OIIO_ImageInput_read_image_format2(OIIO_ImageInput *in,
                                        int chbegin,
                                        int chend,
                                        OIIO_TypeDesc format,
                                        void *data,
                                        stride_t xstride,
                                        stride_t ystride,
                                        stride_t zstride,
                                        void *cbk_data);
const char *OIIO_ImageInput_geterror(const OIIO_ImageInput *in);

// bool ImageInput_read_native_scanline(OIIO_ImageInput *in, int y, int z, void *data);
// bool ImageInput_read_native_tile(OIIO_ImageInput *in, int x, int y, int z, void *data);
// bool ImageInput_read_native_tiles(OIIO_ImageInput *in, int xbegin, int xend, int ybegin, int yend, int zbegin, int zend, void *data);
// bool ImageInput_read_native_deep_scanlines(OIIO_ImageInput *in, int ybegin, int yend, int z, int chbegin, int chend, DeepData* deepdata);
// bool ImageInput_read_native_deep_tiles(OIIO_ImageInput *in, int xbegin, int xend, int ybegin, int yend, int zbegin, int zend,
// 											int chbegin, int chend, DeepData &deepdata);
// bool ImageInput_read_native_deep_image(OIIO_ImageInput *in, DeepData* deepdata);
// int ImageInput_send_to_input(OIIO_ImageInput *in, const char *format,...);
// int ImageInput_send_to_client(OIIO_ImageInput *in, const char *format,...);

//---------------------------------------------------------------------
// OIIO_ImageOutput
//

void OIIO_ImageOutput_delete(OIIO_ImageOutput *out);
OIIO_ImageOutput *OIIO_ImageOutput_create(OIIO_StringRef filename, OIIO_StringRef plugin_searchpath);
const char *OIIO_ImageOutput_format_name(const OIIO_ImageOutput *out);
const OIIO_ImageSpec *OIIO_ImageOutput_spec(const OIIO_ImageOutput *out);
bool OIIO_ImageOutput_supports(const OIIO_ImageOutput *out, OIIO_StringRef feature);
const char *OIIO_ImageOutput_geterror(const OIIO_ImageOutput *out);
bool OIIO_ImageOutput_open(OIIO_ImageOutput *out, OIIO_StringRef name, const OIIO_ImageSpec *newspec,
                           OIIO_ImageOutput_OpenMode openmode);
bool
OIIO_ImageOutput_open2(OIIO_ImageOutput *out, OIIO_StringRef name, int subimages, const OIIO_ImageSpec *const *specs);
bool OIIO_ImageOutput_close(OIIO_ImageOutput *out);
bool OIIO_ImageOutput_write_image(OIIO_ImageOutput *out,
                                  OIIO_TypeDesc format,
                                  const void *data,
                                  ptrdiff_t xstride,
                                  ptrdiff_t ystride,
                                  ptrdiff_t zstride);

//---------------------------------------------------------------------
// OIIO_ImageSpec
//

void OIIO_ImageSpec_delete(OIIO_ImageSpec *spec);

OIIO_ImageSpec *OIIO_ImageSpec_new(OIIO_TypeDesc fmt);
OIIO_ImageSpec *OIIO_ImageSpec_clone(OIIO_ImageSpec *from);
OIIO_ImageSpec *
OIIO_ImageSpec_new_2d(int xres, int yres, int nchans, bool separateformats, const OIIO_TypeDesc *channelformats,
                      const OIIO_StringRef *channelnames);

void OIIO_ImageSpec_default_channel_names(OIIO_ImageSpec *spec);
size_t OIIO_ImageSpec_channel_bytes(const OIIO_ImageSpec *spec);
size_t OIIO_ImageSpec_channel_bytes_chan(const OIIO_ImageSpec *spec, int chan, bool native);
size_t OIIO_ImageSpec_pixel_bytes(const OIIO_ImageSpec *spec, bool native);
size_t OIIO_ImageSpec_pixel_bytes_chans(const OIIO_ImageSpec *spec, int chbegin, int chend, bool native);
imagesize_t OIIO_ImageSpec_scanline_bytes(const OIIO_ImageSpec *spec, bool native);
imagesize_t OIIO_ImageSpec_tile_pixels(const OIIO_ImageSpec *spec);
imagesize_t OIIO_ImageSpec_tile_bytes(const OIIO_ImageSpec *spec, bool native);
imagesize_t OIIO_ImageSpec_image_pixels(const OIIO_ImageSpec *spec);
imagesize_t OIIO_ImageSpec_image_bytes(const OIIO_ImageSpec *spec, bool native);
bool OIIO_ImageSpec_size_safe(const OIIO_ImageSpec *spec);
// std::string metadata_val(const ImageIOParameter &p, bool human=false);
char *OIIO_ImageSpec_to_xml(OIIO_ImageSpec *spec);
// void from_xml(const char *xml)
// bool valid_tile_range(int xbegin, int xend, int ybegin, int yend, int zbegin, int zend)

// void OIIO_ImageSpec_get_channelformats(OIIO_ImageSpec *spec, std::vector< OIIO_TypeDesc > &formats);

// Properties
int OIIO_ImageSpec_x(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_x(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_y(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_y(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_z(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_z(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_width(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_width(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_height(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_height(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_depth(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_depth(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_full_x(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_full_x(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_full_y(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_full_y(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_full_z(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_full_z(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_full_width(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_full_width(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_full_height(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_full_height(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_full_depth(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_full_depth(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_tile_width(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_tile_width(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_tile_height(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_tile_height(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_tile_depth(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_tile_depth(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_nchannels(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_nchannels(OIIO_ImageSpec *spec, int val);
OIIO_TypeDesc OIIO_ImageSpec_format(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_format(OIIO_ImageSpec *spec, OIIO_TypeDesc format);
//int OIIO_ImageSpec_nchannelformats(const OIIO_ImageSpec *spec);
//const OIIO_TypeDesc* OIIO_ImageSpec_channelformats(const OIIO_ImageSpec *spec);
//void OIIO_ImageSpec_set_channelformats(OIIO_ImageSpec *spec, OIIO_TypeDesc *formats);
const char *OIIO_ImageSpec_channelname(const OIIO_ImageSpec *spec, int index);
OIIO_TypeDesc OIIO_ImageSpec_channelformat(const OIIO_ImageSpec *spec, int chan);
//void OIIO_ImageSpec_channelnames(const OIIO_ImageSpec *spec, char **out);
void OIIO_ImageSpec_set_channelnames(OIIO_ImageSpec *spec, char **names);
int OIIO_ImageSpec_alpha_channel(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_alpha_channel(OIIO_ImageSpec *spec, int val);
int OIIO_ImageSpec_z_channel(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_z_channel(OIIO_ImageSpec *spec, int val);
bool OIIO_ImageSpec_deep(const OIIO_ImageSpec *spec);
void OIIO_ImageSpec_set_deep(OIIO_ImageSpec *spec, bool val);

void OIIO_ImageSpec_attribute_type_data(OIIO_ImageSpec *spec, const char *name, OIIO_TypeDesc type, const void *value);
void OIIO_ImageSpec_attribute_type_char(OIIO_ImageSpec *spec, const char *name, OIIO_TypeDesc type, const char *value);
void OIIO_ImageSpec_attribute_uint(OIIO_ImageSpec *spec, const char *name, unsigned int value);
void OIIO_ImageSpec_attribute_int(OIIO_ImageSpec *spec, const char *name, int value);
void OIIO_ImageSpec_attribute_float(OIIO_ImageSpec *spec, const char *name, float value);
void OIIO_ImageSpec_attribute_char(OIIO_ImageSpec *spec, const char *name, const char *value);
int OIIO_ImageSpec_get_int_attribute(OIIO_ImageSpec *spec, const char *name, int defaultval);
float OIIO_ImageSpec_get_float_attribute(OIIO_ImageSpec *spec, const char *name, float defaultval);
const char *OIIO_ImageSpec_get_string_attribute(OIIO_ImageSpec *spec, const char *name, const char *defaultval);
void
OIIO_ImageSpec_erase_attribute(OIIO_ImageSpec *spec, const char *name, OIIO_TypeDesc searchtype, bool caseSensitive);
// ImageIOParameter * find_attribute(const char* name, OIIO_TypeDesc searchtype=OIIO_TypeDesc::UNKNOWN, bool casesensitive=false)
// const ImageIOParameter * find_attribute(const char* name, OIIO_TypeDesc searchtype=OIIO_TypeDesc::UNKNOWN, bool casesensitive=false);



// ImageBuf
//
/*
ImageBuf* ImageBuf_New();
ImageBuf* ImageBuf_New_WithCache(const char* name, ImageCache *imagecache);
ImageBuf* ImageBuf_New_WithBuffer(const char* name, const OIIO_ImageSpec* spec, void *buffer);
ImageBuf* ImageBuf_New_SubImage(const char* name, int subimage, int miplevel, ImageCache* imagecache);
ImageBuf* ImageBuf_New_Spec(const OIIO_ImageSpec* spec);

void ImageBuf_delete(ImageBuf* buf);

void ImageBuf_clear(ImageBuf* buf);
void ImageBuf_reset_subimage(ImageBuf* buf, const char* name, int subimage, int miplevel, ImageCache *imagecache);
void ImageBuf_reset_name_cache(ImageBuf* buf, const char* name, ImageCache *imagecache);
void ImageBuf_reset_spec(ImageBuf* buf, OIIO_ImageSpec* spec);
void ImageBuf_reset_name_spec(ImageBuf* buf, const char* name, const OIIO_ImageSpec* spec);

IBStorage ImageBuf_storage(ImageBuf* buf);
bool ImageBuf_initialized(ImageBuf* buf);
bool ImageBuf_read(ImageBuf* buf, int subimage, int miplevel, bool force, OIIO_TypeDesc convert, void *cbk_data);
bool ImageBuf_init_spec(ImageBuf* buf, const char* filename, int subimage, int miplevel);
bool ImageBuf_write_file(ImageBuf* buf, const char* filename, const char* fileformat, void *cbk_data);
bool ImageBuf_write_output(ImageBuf* buf, OIIO_ImageOutput *out, void *cbk_data);
void ImageBuf_set_write_format(ImageBuf* buf, OIIO_TypeDesc format);
void ImageBuf_set_write_tiles(ImageBuf* buf, int width, int height, int depth);
void ImageBuf_copy_metadata(ImageBuf* dst, const ImageBuf* src);
bool ImageBuf_copy_pixels(ImageBuf* dst, const ImageBuf* src);
bool ImageBuf_copy(ImageBuf* dst, const ImageBuf* src);
void ImageBuf_swap(ImageBuf* buf, ImageBuf* other);
char* ImageBuf_geterror(ImageBuf* buf);
const OIIO_ImageSpec* ImageBuf_spec(ImageBuf* buf);
OIIO_ImageSpec* ImageBuf_specmod(ImageBuf* buf);
const OIIO_ImageSpec* ImageBuf_nativespec(ImageBuf* buf);
const char* ImageBuf_name(ImageBuf* buf);
const char* ImageBuf_file_format_name(ImageBuf* buf);
int ImageBuf_subimage(ImageBuf* buf);
int ImageBuf_nsubimages(ImageBuf* buf);
int ImageBuf_miplevel(ImageBuf* buf);
int ImageBuf_nmiplevels(ImageBuf* buf);
int ImageBuf_nchannels(ImageBuf* buf);
// float ImageBuf_getchannel(ImageBuf* buf, int x, int y, int z, int c, WrapMode wrap);
// void ImageBuf_getpixel(ImageBuf* buf, int x, int y, float *pixel, int maxchannels);
// void ImageBuf_getpixel_xyz(ImageBuf* buf, int x, int y, int z, float *pixel, int maxchannels, WrapMode wrap);
// void ImageBuf_interppixel(ImageBuf* buf, float x, float y, float *pixel, WrapMode wrap);
// void ImageBuf_interppixel_NDC(ImageBuf* buf, float s, float t, float *pixel, WrapMode wrap);
// void ImageBuf_interppixel_NDC_full(ImageBuf* buf, float s, float t, float *pixel, WrapMode wrap);
// void ImageBuf_setpixel(ImageBuf* buf, int x, int y, const float *pixel, int maxchannels);
// void ImageBuf_setpixel_xyz(ImageBuf* buf, int x, int y, int z, const float *pixel, int maxchannels);
// void ImageBuf_setpixel_index(ImageBuf* buf, int i, const float *pixel, int maxchannels);
bool ImageBuf_get_pixel_channels(ImageBuf* buf, int xbegin, int xend, int ybegin, int yend, int zbegin, int zend, int chbegin, int chend, OIIO_TypeDesc format, void *result);
// bool ImageBuf_get_pixels(ImageBuf* buf, int xbegin, int xend, int ybegin, int yend, int zbegin, int zend, OIIO_TypeDesc format, void *result);

int ImageBuf_orientation(ImageBuf* buf);
int ImageBuf_oriented_width(ImageBuf* buf);
int ImageBuf_oriented_height(ImageBuf* buf);
int ImageBuf_oriented_x(ImageBuf* buf);
int ImageBuf_oriented_y(ImageBuf* buf);
int ImageBuf_oriented_full_width(ImageBuf* buf);
int ImageBuf_oriented_full_height(ImageBuf* buf);
int ImageBuf_oriented_full_x(ImageBuf* buf);
int ImageBuf_oriented_full_y(ImageBuf* buf);

int ImageBuf_xbegin(ImageBuf* buf);
int ImageBuf_xend(ImageBuf* buf);
int ImageBuf_ybegin(ImageBuf* buf);
int ImageBuf_yend(ImageBuf* buf);
int ImageBuf_zbegin(ImageBuf* buf);
int ImageBuf_zend(ImageBuf* buf);
int ImageBuf_xmin(ImageBuf* buf);
int ImageBuf_xmax(ImageBuf* buf);
int ImageBuf_ymin(ImageBuf* buf);
int ImageBuf_ymax(ImageBuf* buf);
int ImageBuf_zmin(ImageBuf* buf);
int ImageBuf_zmax(ImageBuf* buf);

void ImageBuf_set_full(ImageBuf* buf, int xbegin, int xend, int ybegin, int yend, int zbegin, int zend);
// void ImageBuf_set_full_border(ImageBuf* buf, int xbegin, int xend, int ybegin, int yend, int zbegin, int zend, const float *bordercolor);

ROI* ImageBuf_roi(ImageBuf* buf);
ROI* ImageBuf_roi_full(ImageBuf* buf);
void ImageBuf_set_roi_full(ImageBuf* buf, ROI* newroi);

bool ImageBuf_pixels_valid(ImageBuf* buf);
OIIO_TypeDesc ImageBuf_pixeltype(ImageBuf* buf);
// void* ImageBuf_localpixels(ImageBuf* buf);
// const void* ImageBuf_localpixels(ImageBuf* buf);
bool ImageBuf_cachedpixels(ImageBuf* buf);
ImageCache* ImageBuf_imagecache(ImageBuf* buf);
// void* ImageBuf_pixeladdr(ImageBuf* buf, int x, int y);
// void* ImageBuf_pixeladdr_z(ImageBuf* buf, int x, int y, int z);
bool ImageBuf_deep(ImageBuf* buf);
// int ImageBuf_deep_samples(ImageBuf* buf, int x, int y, int z);
// const void* ImageBuf_deep_pixel_ptr(ImageBuf* buf, int x, int y, int z, int c);
// float ImageBuf_deep_value(ImageBuf* buf, int x, int y, int z, int c, int s);
// DeepData* ImageBuf_deepdata(ImageBuf* buf);

// ROI
//
void ROI_delete(ROI* roi);

ROI* ROI_New();
ROI* ROI_NewOptions(int xbeing, int xend, int ybegin, int yend, int zbegin, int zend, int chbegin, int chend);
ROI* ROI_Copy(const ROI *roi);

bool ROI_defined(ROI* roi);
int ROI_width(ROI* roi);
int ROI_height(ROI* roi);
int ROI_depth(ROI* roi);
int ROI_nchannels(ROI* roi);
imagesize_t ROI_npixels(ROI* roi);

// Properties
int ROI_xbegin(const ROI* roi);
void ROI_set_xbegin(ROI* roi, int val);
int ROI_xend(const ROI* roi);
void ROI_set_xend(ROI* roi, int val);
int ROI_ybegin(const ROI* roi);
void ROI_set_ybegin(ROI* roi, int val);
int ROI_yend(const ROI* roi);
void ROI_set_yend(ROI* roi, int val);
int ROI_zbegin(const ROI* roi);
void ROI_set_zbegin(ROI* roi, int val);
int ROI_zend(const ROI* roi);
void ROI_set_zend(ROI* roi, int val);
int ROI_chbegin(const ROI* roi);
void ROI_set_chbegin(ROI* roi, int val);
int ROI_chend(const ROI* roi);
void ROI_set_chend(ROI* roi, int val);
*/

// ImageCache
//


OIIO_ImageCache *OIIO_ImageCache_create(bool shared);
void OIIO_ImageCache_destroy(OIIO_ImageCache *x, bool teardown);
void OIIO_ImageCache_clear(OIIO_ImageCache *x);
const char *OIIO_ImageCache_geterror(const OIIO_ImageCache *x);
const char *OIIO_ImageCache_getstats(const OIIO_ImageCache *x, int level);
void OIIO_ImageCache_reset_stats(OIIO_ImageCache *x);
void OIIO_ImageCache_invalidate(OIIO_ImageCache *x, OIIO_StringRef filename);
void OIIO_ImageCache_invalidate_all(OIIO_ImageCache *x, bool force);
bool OIIO_ImageCache_attribute(OIIO_ImageCache *x, OIIO_StringRef name, OIIO_TypeDesc type, const void *val);
bool OIIO_ImageCache_getattribute(OIIO_ImageCache *x, OIIO_StringRef name, OIIO_TypeDesc type, void *val);
OIIO_ImageCache_Perthread *
OIIO_ImageCache_get_perthread_info(OIIO_ImageCache *x, OIIO_ImageCache_Perthread *thread_info);
OIIO_ImageCache_Perthread *OIIO_ImageCache_create_perthread_info(OIIO_ImageCache *x);
void OIIO_ImageCache_destroy_perthread_info(OIIO_ImageCache *x, OIIO_ImageCache_Perthread *thread_info);
OIIO_ImageCache_ImageHandle *OIIO_ImageCache_get_image_handle(OIIO_ImageCache *x, OIIO_StringRef name);
bool OIIO_ImageCache_good(OIIO_ImageCache *x, OIIO_ImageCache_ImageHandle *file);
bool OIIO_ImageCache_get_image_info(OIIO_ImageCache *x, OIIO_StringRef filename, int subimage, int miplevel,
                                    OIIO_StringRef dataname, OIIO_TypeDesc datatype, void *data);
bool OIIO_ImageCache_get_image_info_by_handle(OIIO_ImageCache *x, OIIO_ImageCache_ImageHandle *file,
                                              OIIO_ImageCache_Perthread *thread_info, int subimage, int miplevel,
                                              OIIO_StringRef dataname, OIIO_TypeDesc datatype, void *data);
bool OIIO_ImageCache_get_imagespec(OIIO_ImageCache *x, OIIO_StringRef filename, OIIO_ImageSpec *spec,
                                   int subimage, int miplevel, bool native);
bool OIIO_ImageCache_get_imagespec_by_handle(OIIO_ImageCache *x, OIIO_ImageCache_ImageHandle *file,
                                             OIIO_ImageCache_Perthread *thread_info,
                                             OIIO_ImageSpec *spec, int subimage, int miplevel, bool native);
const char *OIIO_ImageCache_resolve_filename(OIIO_ImageCache *x, OIIO_StringRef filename);
bool OIIO_ImageCache_get_pixels(OIIO_ImageCache *x,
                                OIIO_StringRef filename,
                                int subimage, int miplevel,
                                int xbegin, int xend,
                                int ybegin, int yend,
                                int zbegin, int zend,
                                OIIO_TypeDesc format, void *result);
bool OIIO_ImageCache_get_pixels_by_handle(OIIO_ImageCache *x,
                                 OIIO_ImageCache_ImageHandle *file,
                                 OIIO_ImageCache_Perthread *thread_info,
                                 int subimage, int miplevel,
                                 int xbegin, int xend,
                                 int ybegin, int yend,
                                 int zbegin, int zend,
                                 OIIO_TypeDesc format, void *result);
bool OIIO_ImageCache_get_pixels_stride(OIIO_ImageCache *x,
                                 OIIO_StringRef filename, int subimage, int miplevel,
                                 int xbegin, int xend, int ybegin, int yend,
                                 int zbegin, int zend, int chbegin, int chend,
                                 OIIO_TypeDesc format, void *result,
                                 stride_t xstride, stride_t ystride, stride_t zstride,
                                 int cache_chbegin, int cache_chend);
bool OIIO_ImageCache_get_pixels_stride_by_handle(OIIO_ImageCache *x, OIIO_ImageCache_ImageHandle *file,
                                 OIIO_ImageCache_Perthread *thread_info,
                                 int subimage, int miplevel,
                                 int xbegin, int xend, int ybegin, int yend,
                                 int zbegin, int zend, int chbegin, int chend,
                                 OIIO_TypeDesc format, void *result,
                                 stride_t xstride, stride_t ystride, stride_t zstride,
                                 int cache_chbegin, int cache_chend);

// bool ImageCache_attribute(ImageCache *x, const char *name, OIIO_TypeDesc type, const void *val);

// bool ImageCache_attribute_int(ImageCache *x, const char *name, int val);
// bool ImageCache_attribute_float(ImageCache *x, const char *name, float val);
// bool ImageCache_attribute_double(ImageCache *x, const char *name, double val);
// bool ImageCache_attribute_char(ImageCache *x, const char *name, const char **val);

// bool ImageCache_getattribute(ImageCache *x, const char *name, OIIO_TypeDesc type, void *val);
// bool ImageCache_getattribute_int(ImageCache *x, const char *name, int *val);
// bool ImageCache_getattribute_float(ImageCache *x, const char *name, float *val);
// bool ImageCache_getattribute_double(ImageCache *x, const char *name, double *val);
// bool ImageCache_getattribute_char(ImageCache *x, const char *name, char **val);

// char* ImageCache_resolve_filename(ImageCache *x, const char *filename);

// bool ImageCache_get_image_info(ImageCache *x, char *filename, int subimage, int miplevel,
//                      			char *dataname, OIIO_TypeDesc datatype, void *data);

// bool ImageCache_get_imagespec(ImageCache *x, char *filename, OIIO_ImageSpec &spec,
//                             int subimage=0, int miplevel=0,
//                             bool native=false);

// const OIIO_ImageSpec* ImageCache_imagespec(ImageCache *x, char *filename, int subimage, int miplevel, bool native);

// bool get_pixels(ImageCache *x, char *filename, int subimage, int miplevel,
//                          int xbegin, int xend, int ybegin, int yend,
//                          int zbegin, int zend,
//                          OIIO_TypeDesc format, void *result);

// bool get_pixels(ImageCache *x, char *filename,
//                 int subimage, int miplevel, int xbegin, int xend,
//                 int ybegin, int yend, int zbegin, int zend,
//                 int chbegin, int chend, OIIO_TypeDesc format, void *result,
//                 stride_t xstride=AutoStride, stride_t ystride=AutoStride,
//                 stride_t zstride=AutoStride);

// Tile* ImageCache_get_tile(ImageCache *x, char *filename, int subimage, int miplevel,
//                             int x, int y, int z);

// void ImageCache_release_tile(ImageCache *x, Tile *tile);
// const void* ImageCache_tile_pixels(ImageCache *x, Tile *tile, OIIO_TypeDesc *format);
// bool ImageCache_add_file(ImageCache *x, char *filename, OIIO_ImageInput::Creator creator);
// bool ImageCache_add_tile(ImageCache *x, char *filename, int subimage, int miplevel,
// 		                 int x, int y, int z, OIIO_TypeDesc format, const void *buffer,
// 		                 stride_t xstride=AutoStride, stride_t ystride=AutoStride,
// 		                 stride_t zstride=AutoStride);



#ifdef __cplusplus
}
#endif

#endif