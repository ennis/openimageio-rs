#include "oiio.h"
#include "helpers.hpp"
#include <OpenImageIO/imageio.h>

extern "C" {
    const char* OIIO_geterror() {
        return makeCString(OIIO::geterror());
    }

    void OIIO_freeString(const char *ptr) {
        freeCString(ptr);
    }
}