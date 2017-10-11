#include <OpenImageIO/imageio.h>

extern "C" {
OIIO::ImageSpec *COIIO_ImageSpec_new(const OIIO::TypeDesc *format);
OIIO::ImageSpec *COIIO_ImageSpec_new_2D(int xres, int yres, int nchans,
                                        const OIIO::TypeDesc *fmt);
void COIIO_ImageSpec_delete(OIIO::ImageSpec *ptr);

int COIIO_ImageSpec_x(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_x(OIIO::ImageSpec *this_, int x);
int COIIO_ImageSpec_y(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_y(OIIO::ImageSpec *this_, int y);
int COIIO_ImageSpec_z(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_z(OIIO::ImageSpec *this_, int z);

int COIIO_ImageSpec_full_x(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_full_x(OIIO::ImageSpec *this_, int x);
int COIIO_ImageSpec_full_y(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_full_y(OIIO::ImageSpec *this_, int y);
int COIIO_ImageSpec_full_z(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_full_z(OIIO::ImageSpec *this_, int z);

int COIIO_ImageSpec_width(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_width(OIIO::ImageSpec *this_, int width);
int COIIO_ImageSpec_height(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_height(OIIO::ImageSpec *this_, int height);
int COIIO_ImageSpec_depth(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_depth(OIIO::ImageSpec *this_, int depth);

int COIIO_ImageSpec_full_width(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_full_width(OIIO::ImageSpec *this_, int width);
int COIIO_ImageSpec_full_height(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_full_height(OIIO::ImageSpec *this_, int height);
int COIIO_ImageSpec_full_depth(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_full_depth(OIIO::ImageSpec *this_, int depth);

int COIIO_ImageSpec_tile_width(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_tile_width(OIIO::ImageSpec *this_, int width);
int COIIO_ImageSpec_tile_height(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_tile_height(OIIO::ImageSpec *this_, int height);
int COIIO_ImageSpec_tile_depth(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_tile_depth(OIIO::ImageSpec *this_, int depth);
int COIIO_ImageSpec_nchannels(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_nchannels(OIIO::ImageSpec *this_, int nchannels);
const OIIO::TypeDesc *COIIO_ImageSpec_format(const OIIO::ImageSpec *this_);
void COIIO_ImageSpec_set_format(OIIO::ImageSpec *this_,
                                const OIIO::TypeDesc *format);
// TypeDesc
const char *COIIO_TypeDesc_c_str(const OIIO::TypeDesc *this_);
size_t COIIO_TypeDesc_fromstring(OIIO::TypeDesc *this_, const char *typestring);
// ImageInput
OIIO::ImageInput *COIIO_ImageInput_open(const char *filename,
                                        const OIIO::ImageSpec *config);
void COIIO_ImageInput_close(OIIO::ImageInput *this_);
void COIIO_ImageInput_destroy(OIIO::ImageInput *input);
const OIIO::ImageSpec *COIIO_ImageInput_spec(const OIIO::ImageInput *this_);
bool COIIO_ImageInput_read_image(OIIO::ImageInput *this_,
                                 const OIIO::TypeDesc *format, void *data);
// global error handling
int COIIO_geterror(char *buf, int bufsize);
// ImageOutput
OIIO::ImageOutput *COIIO_ImageOutput_create(const char *filename,
                                            const char *plugin_searchpath);
bool COIIO_ImageOutput_open(
    OIIO::ImageOutput *this_, const char *filename, const OIIO::ImageSpec *spec,
    OIIO::ImageOutput::OpenMode mode);
bool COIIO_ImageOutput_open_with_subimages(
    OIIO::ImageOutput *this_, const char *filename, int num_subimages,
    const OIIO::ImageSpec *subimage_specs);
int COIIO_ImageOutput_supports(const OIIO::ImageOutput *this_,
                               const char *feature);
bool COIIO_ImageOutput_write_image(OIIO::ImageOutput *this_,
                                   const OIIO::TypeDesc *format,
                                   const void *data, ptrdiff_t xstride,
                                   ptrdiff_t ystride, ptrdiff_t zstride);
bool COIIO_ImageOutput_close(OIIO::ImageOutput *this_);
void COIIO_ImageOutput_destroy(OIIO::ImageOutput *imageout);
}