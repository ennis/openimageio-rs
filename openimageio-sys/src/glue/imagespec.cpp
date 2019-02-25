#include <OpenImageIO/imageio.h>
#include <string>
#include "oiio.h"
#include "helpers.hpp"

extern "C" {

void OIIO_ImageSpec_delete(OIIO_ImageSpec *spec) {
    delete OIIO_CAST(ImageSpec ,spec);
}

OIIO_ImageSpec *OIIO_ImageSpec_new(OIIO_TypeDesc fmt) {
    return (OIIO_ImageSpec *) new OIIO::ImageSpec(unwrapTypeDesc(fmt));
}

OIIO_ImageSpec *OIIO_ImageSpec_clone(OIIO_ImageSpec *from)
{
    return (OIIO_ImageSpec *) new OIIO::ImageSpec(*OIIO_CAST(ImageSpec, from));
}

OIIO_ImageSpec *
OIIO_ImageSpec_new_2d(int xres, int yres, int nchans, bool separateformats, const OIIO_TypeDesc *channelformats,
                      const OIIO_StringRef *channelnames) {
    auto spec = new OIIO::ImageSpec(xres, yres, nchans, unwrapTypeDesc(*channelformats));
    for (int i = 0; i < nchans; ++i) {
        spec->channelnames.push_back(std::string{channelnames[i].ptr, channelnames[i].len});
        if (separateformats) {
            spec->channelformats.push_back(unwrapTypeDesc(channelformats[i]));
        }
    }

    return (OIIO_ImageSpec *) spec;
}

OIIO_ImageSpec *OIIO_ImageSpec_new_size(int xres, int yres, int nchans, OIIO_TypeDesc fmt) {
    auto fmt2 = *OIIO_CAST_CONST(TypeDesc ,&fmt);
    return (OIIO_ImageSpec *) new OIIO::ImageSpec(xres, yres, nchans, fmt2);
}

void OIIO_ImageSpec_default_channel_names(OIIO_ImageSpec *spec) {
    OIIO_CAST(ImageSpec,spec)->default_channel_names();
}

size_t OIIO_ImageSpec_channel_bytes(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec,spec)->channel_bytes();
}

size_t OIIO_ImageSpec_channel_bytes_chan(const OIIO_ImageSpec *spec, int chan, bool native) {
    return OIIO_CAST_CONST(ImageSpec,spec)->channel_bytes(chan, native);
}

size_t OIIO_ImageSpec_pixel_bytes(const OIIO_ImageSpec *spec, bool native) {
    return OIIO_CAST_CONST(ImageSpec,spec)->pixel_bytes(native);
}

size_t OIIO_ImageSpec_pixel_bytes_chans(const OIIO_ImageSpec *spec, int chbegin, int chend, bool native) {
    return OIIO_CAST_CONST(ImageSpec,spec)->pixel_bytes(chbegin, chend, native);
}

imagesize_t OIIO_ImageSpec_scanline_bytes(const OIIO_ImageSpec *spec, bool native) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->scanline_bytes(native);
}

imagesize_t OIIO_ImageSpec_tile_pixels(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec,spec)->tile_pixels();
}

imagesize_t OIIO_ImageSpec_tile_bytes(const OIIO_ImageSpec *spec, bool native) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->tile_bytes(native);
}

imagesize_t OIIO_ImageSpec_image_pixels(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->image_pixels();
}

imagesize_t OIIO_ImageSpec_image_bytes(const OIIO_ImageSpec *spec, bool native) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->image_bytes(native);
}

bool OIIO_ImageSpec_size_safe(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->size_t_safe();
}

// Properties
int OIIO_ImageSpec_x(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->x;
}

void OIIO_ImageSpec_set_x(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->x = val;
}

int OIIO_ImageSpec_y(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->y;
}

void OIIO_ImageSpec_set_y(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->y = val;
}

int OIIO_ImageSpec_z(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->z;
}

void OIIO_ImageSpec_set_z(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->z = val;
}

int OIIO_ImageSpec_width(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->width;
}

void OIIO_ImageSpec_set_width(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->width = val;
}

int OIIO_ImageSpec_height(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->height;
}

void OIIO_ImageSpec_set_height(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->height = val;
}

int OIIO_ImageSpec_depth(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->depth;
}

void OIIO_ImageSpec_set_depth(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->depth = val;
}

int OIIO_ImageSpec_full_x(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->full_x;
}

void OIIO_ImageSpec_set_full_x(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->full_x = val;
}

int OIIO_ImageSpec_full_y(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->full_y;
}

void OIIO_ImageSpec_set_full_y(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->full_y = val;
}

int OIIO_ImageSpec_full_z(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->full_z;
}

void OIIO_ImageSpec_set_full_z(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->full_z = val;
}

int OIIO_ImageSpec_full_width(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->full_width;
}

void OIIO_ImageSpec_set_full_width(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->full_width = val;
}

int OIIO_ImageSpec_full_height(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->full_height;
}

void OIIO_ImageSpec_set_full_height(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->full_height = val;
}

int OIIO_ImageSpec_full_depth(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->full_depth;
}

void OIIO_ImageSpec_set_full_depth(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->full_depth = val;
}

int OIIO_ImageSpec_tile_width(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->tile_width;
}

void OIIO_ImageSpec_set_tile_width(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->tile_width = val;
}

int OIIO_ImageSpec_tile_height(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->tile_height;
}

void OIIO_ImageSpec_set_tile_height(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->tile_height = val;
}

int OIIO_ImageSpec_tile_depth(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->tile_depth;
}

void OIIO_ImageSpec_set_tile_depth(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->tile_depth = val;
}

int OIIO_ImageSpec_nchannels(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->nchannels;
}

void OIIO_ImageSpec_set_nchannels(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->nchannels = val;
}

OIIO_TypeDesc OIIO_ImageSpec_format(const OIIO_ImageSpec *spec) {
    return wrapTypeDesc(OIIO_CAST_CONST(ImageSpec ,spec)->format);
}

void OIIO_ImageSpec_set_format(OIIO_ImageSpec *spec, OIIO_TypeDesc fmt) {
    OIIO_CAST(ImageSpec ,spec)->set_format(unwrapTypeDesc(fmt));
}

/*
const OIIO_TypeDesc* OIIO_ImageSpec_channelformats(const OIIO_ImageSpec *spec) {
	const std::vector<OIIO::TypeDesc>& vec = OIIO_CAST_CONST(ImageSpec,spec)->channelformats;
	return reinterpret_cast<const OIIO_TypeDesc*>(vec.data());
}*/

const char *OIIO_ImageSpec_channelname(const OIIO_ImageSpec *spec, int index) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->channel_name(index).data();
}

OIIO_TypeDesc OIIO_ImageSpec_channelformat(const OIIO_ImageSpec *spec, int chan) {
    return wrapTypeDesc(OIIO_CAST_CONST(ImageSpec ,spec)->channelformat(chan));
}

/*
void OIIO_ImageSpec_set_channelformats(OIIO_ImageSpec *spec, OIIO_TypeDesc* formats){
	OIIO::ImageSpec *ptr = OIIO_CAST(ImageSpec,spec);
	std::vector<OIIO::TypeDesc> vec = ptr->channelformats;
	for (std::vector<std::string>::size_type i = 0; i != vec.size(); i++) {
		vec[i] = unwrapTypeDesc(formats[i]);
	}
	ptr->channelformats = vec;
}
*/

/*
void OIIO_ImageSpec_channelnames(const OIIO_ImageSpec *spec, char** out) {
	std::vector<std::string> vec = OIIO_CAST_CONST(ImageSpec,spec)->channelnames;
	for (std::vector<std::string>::size_type i = 0; i != vec.size(); i++) {
		out[i] = strdup(vec[i].c_str());
	}
}*/

void OIIO_ImageSpec_set_channelnames(OIIO_ImageSpec *spec, char **names) {
    OIIO::ImageSpec *ptr = OIIO_CAST(ImageSpec ,spec);
    std::vector<std::string> vec = ptr->channelnames;
    for (std::vector<std::string>::size_type i = 0; i != vec.size(); i++) {
        vec[i] = std::string(names[i]);
    }
    ptr->channelnames = vec;
}

int OIIO_ImageSpec_alpha_channel(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->alpha_channel;
}

void OIIO_ImageSpec_set_alpha_channel(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->alpha_channel = val;
}

int OIIO_ImageSpec_z_channel(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->z_channel;
}

void OIIO_ImageSpec_set_z_channel(OIIO_ImageSpec *spec, int val) {
    OIIO_CAST(ImageSpec ,spec)->z_channel = val;
}

bool OIIO_ImageSpec_deep(const OIIO_ImageSpec *spec) {
    return OIIO_CAST_CONST(ImageSpec ,spec)->deep;
}

void OIIO_ImageSpec_set_deep(OIIO_ImageSpec *spec, bool val) {
    OIIO_CAST(ImageSpec ,spec)->deep = val;
}

char *OIIO_ImageSpec_to_xml(OIIO_ImageSpec *spec) {
    std::string s = OIIO_CAST(ImageSpec ,spec)->to_xml();
    return strdup(s.c_str());
}

void OIIO_ImageSpec_attribute_type_data(OIIO_ImageSpec *spec, const char *name, OIIO_TypeDesc type, const void *value) {
    OIIO_CAST(ImageSpec ,spec)->attribute(name, unwrapTypeDesc(type), value);
}

void OIIO_ImageSpec_attribute_type_char(OIIO_ImageSpec *spec, const char *name, OIIO_TypeDesc type, const char *value) {
    OIIO_CAST(ImageSpec ,spec)->attribute(name, unwrapTypeDesc(type), value);
}

void OIIO_ImageSpec_attribute_uint(OIIO_ImageSpec *spec, const char *name, unsigned int value) {
    OIIO_CAST(ImageSpec ,spec)->attribute(name, value);
}

void OIIO_ImageSpec_attribute_int(OIIO_ImageSpec *spec, const char *name, int value) {
    OIIO_CAST(ImageSpec ,spec)->attribute(name, value);
}

void OIIO_ImageSpec_attribute_float(OIIO_ImageSpec *spec, const char *name, float value) {
    OIIO_CAST(ImageSpec ,spec)->attribute(name, value);
}

void OIIO_ImageSpec_attribute_char(OIIO_ImageSpec *spec, const char *name, const char *value) {
    OIIO_CAST(ImageSpec ,spec)->attribute(name, value);
}

void OIIO_ImageSpec_erase_attribute(OIIO_ImageSpec *spec, const char *name, OIIO_TypeDesc type, bool caseSensitive) {
    OIIO_CAST(ImageSpec ,spec)->erase_attribute(name, unwrapTypeDesc(type), caseSensitive);
}

int OIIO_ImageSpec_get_int_attribute(OIIO_ImageSpec *spec, const char *name, int defaultval) {
    return OIIO_CAST(ImageSpec ,spec)->get_int_attribute(name, defaultval);
}

float OIIO_ImageSpec_get_float_attribute(OIIO_ImageSpec *spec, const char *name, float defaultval) {
    return OIIO_CAST(ImageSpec ,spec)->get_float_attribute(name, defaultval);
}

const char *OIIO_ImageSpec_get_string_attribute(OIIO_ImageSpec *spec, const char *name, const char *defaultval) {
    return OIIO_CAST(ImageSpec ,spec)->get_string_attribute(name, defaultval).c_str();
}

} // extern "C"


